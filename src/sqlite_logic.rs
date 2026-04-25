use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};
//use serde_json::{json};

#[derive(Serialize, Deserialize)]
struct TestJson {
    header: String,
    message: String,
}

pub fn main(input: &str, _dbpath: &str) -> Result<String, Box<dyn std::error::Error>> {

    //inputのjsonを変換
    let input_json: TestJson = serde_json::from_str(input).unwrap();

    //DB接続(初回処理)
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS world (
            id INTEGER PRIMARY KEY,
            header TEXT NOT NULL,
            message TEXT NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "INSERT INTO 
            world (
                header, 
                message
            )
        VALUES (
            ?1, 
            ?2
        )",
        ("hello", "hello world!!!!!")
    )?;

    //検索
    //select文作成
    let mut sql: String = 
        "Select 
            header,
            message 
        FROM 
            world 
        WHERE 
            1 = 1".to_string();
    sql.push_str(" AND header = ?");

    //パラメータ追加
    let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
    params.push(&input_json.header);

    //実行
    let mut stmt = conn.prepare(&sql)?;
    let res = stmt.query_map(params.as_slice(), |row| {
        Ok(TestJson {
            header: row.get(0)?,
            message: row.get(1)?,
        })
    })?;
    // let res = stmt.query_map(params.as_slice(), |row| {
    //     Ok(json!({
    //         "header": row.get::<_, String>(0)?,
    //         "message": row.get::<_, String>(1)?,
    //     }))
    // })?;

    //検索結果をjsonに変換
    let vec_res = res.collect::<Result<Vec<_>>>()?;
    let res_json: String = serde_json::to_string(&vec_res).unwrap();

    //結果リターン
    Ok(res_json)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_main() -> Result<(), Box<dyn std::error::Error>> {
        
        //パラメータ作成
        let input: String = serde_json::to_string(&TestJson{
            header: "hello".to_string(),
            message: "".to_string(),
        }).unwrap();
        let dbpath: &str = "test.db";
        
        //テスト実行
        let res = main(&input, dbpath)?;

        //結果判定
        let list: Vec<TestJson> = vec![TestJson{
            header: "hello".to_string(),
            message: "hello world!!!!!".to_string(),
        },];
        let expect: String = serde_json::to_string(list.as_slice()).unwrap();
        assert_eq!(res, expect);
        Ok(())
    }

}