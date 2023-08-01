pub use squirrel::SquirrelBuilder;

pub use boost::BoostBuilder;
pub use mariadb::MariadbBuilder;
pub use mysql::MysqlBuilder;
pub use pgsql::PgsqlBuilder;

mod squirrel;

mod boost;
mod mariadb;
mod mysql;
mod pgsql;

pub trait Builder {
    fn setup() -> Result<(), Box<dyn std::error::Error>>;
}
