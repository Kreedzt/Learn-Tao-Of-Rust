use super::*;

pub fn replace_column(data: String, column: &str, replacement: &str) -> Result<String, Error> {
    let mut lines = data.lines();
    
    let headers = lines.next().unwrap();
    // 分割列
    let columns: Vec<&str> = headers.split(',').collect();
    // 寻找目标列索引
    let column_number = columns.iter().position(|&e| e == column);
    
    let column_number = match column_number {
        Some(column) => column,
        None => Err("column name doesn't exist in the input file")?
    };

    let mut result = String::with_capacity(data.capacity());
    // 重新拼回原始 header
    result.push_str(&columns.join(","));
    result.push('\n');

    // 遍历其余内容行
    for line in lines {
        // 分割列
        let mut records: Vec<&str> = line.split(',').collect();
        records[column_number] = replacement;
        // 替换后拼回字符串
        result.push_str(&records.join(","));
        result.push('\n');
    }

    Ok(result)
}
