use rusqlite::Connection;


pub  fn connect() -> rusqlite::Result<Connection> {
    Connection::open("novafs.db")
}