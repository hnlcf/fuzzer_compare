pub use squirrel::SquirrelRunner;

mod squirrel;

pub trait Runner {
    fn run_pgsql() -> Result<(), Box<dyn std::error::Error>>;
    fn run_mysql() -> Result<(), Box<dyn std::error::Error>>;
    fn run_mariadb() -> Result<(), Box<dyn std::error::Error>>;
}
