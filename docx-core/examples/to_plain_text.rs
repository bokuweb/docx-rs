use docx_rs::Docx;

fn main() {
    let path =
        std::path::Path::new(r"C:\Users\33028\Desktop\全市部分县区统计执法检查报告 (1).docx");
    let docx = Docx::read_file(&path).unwrap();
    let plain_text = docx.to_plain_text();
    println!("{}", plain_text);
}
