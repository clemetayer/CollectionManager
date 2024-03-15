pub fn create_collection(collection_name : String) -> Result<()>{
    let conn = Connection::open("collections.db")?;
    Ok(())
}