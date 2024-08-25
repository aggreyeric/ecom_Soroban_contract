#![no_std]


use soroban_sdk::{
    contract, contractimpl, contracttype, token, vec, Address, Env, Error, String,
     Vec,
};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
struct Sales {
    id: i32,
    product: String,
    person: Address,
    price: i128,
}

// const  MaxProducts: Symbol =  symbol_short!("MaxP");

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataKey {
    Product(i32),
    Admin,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Products {
    id: i32,
    name: String,
    price: i128,
    quantity: i32,
    sales: Vec<Sales>,
}

pub trait EcomContractTrait {
    fn get_totalsales(env: Env) -> i128 ;
    fn cash_out(env: Env, admin: Address, some:Address) -> Result<bool, Error>;
    fn get_product(env: Env, id: i32) -> Result<Products, Error>;
    fn initialize(env: Env, admin: Address) -> Result<bool, Error>;
    fn was_paid(env: Env, product_id: i32, user: Address, sale_id: i32) -> Result<bool, Error>;
    fn sell_product(env: Env, id: i32, person: Address) -> Result<bool, Error>;
    fn add_product(
        env: Env,
        id: i32,
        name: String,
        price: i128,
        quantity: i32,
    ) -> Result<bool, Error>;
}

#[contract]
pub struct EcomContract;

#[contractimpl]
impl EcomContractTrait for EcomContract {

    fn cash_out(env: Env, admin: Address, some:Address) -> Result<bool, Error> {
        admin.require_auth();
        let totalsales = &totalsales(&env);
        transfer_to_another_address(&env, &some, &totalsales);
        Ok(true)
    }


    fn get_totalsales(env: Env) -> i128 {
        let admin_data: Address = env.clone().storage().instance().get(&DataKey::Admin).unwrap();
        let client = token::Client::new(&env, &admin_data);
        client.balance(&get_native_asset_address(&env))
    }

    fn get_product(env: Env, id: i32) -> Result<Products, Error> {
        assert!(env.storage().instance().has(&DataKey::Product(id)), "Product not found");
        let product: Products = env.storage().instance().get(&DataKey::Product(id)).unwrap();
        Ok(product)
    }

    fn was_paid(env: Env, product_id: i32, user: Address, sale_id: i32) -> Result<bool, Error> {
        let product: Products = env.storage().instance().get(&DataKey::Product(product_id)).unwrap();
        let sales: Vec<Sales> = product.sales.clone();
        for sale in sales {
            if sale.id == sale_id && sale.person == user {
                return Ok(true);
            }
        }
        Ok(false)
    }



    fn initialize(env: Env, admin: Address) -> Result<bool, Error> {
        admin.require_auth();
        assert!(
            !env.storage().instance().has(&DataKey::Admin),
            "Already initialized"
        );

        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(true)
    }

    fn add_product(
        env: Env,
        id: i32,
        name: String,
        price: i128,
        quantity: i32,
    ) -> Result<bool, Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("Admin not initialized");

        admin.require_auth();

        assert!(
            !env.storage().instance().has(&DataKey::Product(id)),
            "Product already exists"
        );

        let product = Products {
            id,
            name,
            price,
            quantity,
            sales: vec![&env],
        };

        env.storage()
            .instance()
            .set(&DataKey::Product(id), &product);

        Ok(true)
    }

    fn sell_product(env: Env, id: i32, person: Address) -> Result<bool, Error> {
        person.require_auth();
        let mut product: Products = env.storage().instance().get(&DataKey::Product(id)).unwrap();
        let price = product.price;

        // Transfer tokens from the user to the contract
        user_pay_to_contract(&env, &person, price);

        product.sales.push_back(Sales {
            id,
            product: product.name.clone(),
            person,
            price,
        });

        env.storage()
            .instance()
            .set(&DataKey::Product(id), &product);

        Ok(true)
    }
}

///
///

pub(crate) fn get_native_asset_address(env: &Env) -> Address {
    let string_adr: String = String::from_str(
        &env,
        "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC",
    );
    Address::from_string(&string_adr)
}

// Transfer tokens from the contract to the recipient
pub(crate) fn transfer_to_another_address(env: &Env, to: &Address, amount: &i128) {
    let admin_data: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    let client = token::Client::new(env, &admin_data);
    client.transfer(&env.current_contract_address(), to, amount);
}

// Transfer tokens from the poster to the contract
pub(crate) fn user_pay_to_contract(env: &Env, user: &Address, price: i128) {
    let admin_data: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    let client = token::Client::new(env, &admin_data);
    client.transfer(user, &env.current_contract_address(), &price);
}


pub(crate) fn totalsales(env: &Env) -> i128 {
    let admin_data: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    let client = token::Client::new(env, &admin_data);
    client.balance(&env.current_contract_address())
}


mod test;


//CA4NABT6A4QMAQQ75TN6FUOL7PYIGR6UCKK2WAMZ2H5NITJK7OO34VRU