#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Vec, vec, Symbol, contracttype};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
enum OperationType {
    Sum,
    Sub,
    Mul,
    Div,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
struct Operation {
    x: u32,
    y: u32,
    z: u32,
    op: OperationType,
    id: u32,
}

#[contract]
pub struct MathContract;

const ID: Symbol = symbol_short!("ID");
const ALL_OPS: Symbol = symbol_short!("ALL_OPS");
const LAST_OP: Symbol = symbol_short!("LAST_OP");

impl Default for Operation {
    fn default() -> Self {
        Operation {
            x: 0,
            y: 0,
            z: 0,
            op: OperationType::Sum,
            id: 0,
        }
    }
}

#[contractimpl]
impl MathContract {

    fn get_op_id(env: Env) -> u32 {
        let mut id: u32 = env.storage().instance().get(&ID).unwrap_or(0);
        id += 1;
        env.storage().instance().set(&ID, &id);
        env.storage().instance().extend_ttl(100, 100);
        id
    }

    fn store_op(env: Env, operation: Operation) {
        let mut all_ops: Vec<Operation> = env.storage().instance().get(&ALL_OPS).unwrap_or(Vec::new(&Default::default()));
        let op_clone = operation.clone();
        all_ops.append(&mut vec![&env, op_clone]);
        env.storage().instance().set(&ALL_OPS, &all_ops);
        env.storage().instance().set(&LAST_OP, &operation);
    }

    pub fn last_op(env: Env) -> Operation {
        env.storage().instance().get(&LAST_OP).unwrap_or_default()
    }

    pub fn all_op(env: Env) -> Vec<Operation> {
        env.storage().instance().get(&ALL_OPS).unwrap_or(Vec::new(&Default::default()))
    }

    pub fn sum(env: Env, x: u32, y: u32) -> u32 {
        let z = x.saturating_add(y);
        let operation = Operation {
            x,
            y,
            z,
            op: OperationType::Sum,
            id: Self::get_op_id(env.clone()),
        };
        Self::store_op(env, operation);
        z
    }

    pub fn sub(env: Env, x: u32, y: u32) -> u32 {
        let z = if y >= x { y - x } else { x - y };
        let operation = Operation {
            x,
            y,
            z,
            op: OperationType::Sub,
            id: Self::get_op_id(env.clone()),
        };
        Self::store_op(env, operation);
        z
    }

    pub fn mul(env: Env, x: u32, y: u32) -> u32 {
        let z = x * y;
        let operation = Operation {
            x,
            y,
            z,
            op: OperationType::Mul,
            id: Self::get_op_id(env.clone()),
        };
        Self::store_op(env, operation);
        x * y
    }

    pub fn div(env: Env, x: u32, y: u32) -> u32 {
        let z = if y == 0 { u32::MAX } else { x / y };
        let operation = Operation {
            x,
            y,
            z,
            op: OperationType::Div,
            id: Self::get_op_id(env.clone()),
        };
        Self::store_op(env, operation);
        z
    }
}

mod test;
