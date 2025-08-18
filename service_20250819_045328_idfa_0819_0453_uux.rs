use rocket::get;
use rocket::Route;
use std::path::{Path, PathBuf};
use std::fs::{self, DirEntry};
# FIXME: 处理边界情况
use std::io;

/// 主程序入口
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

#[derive(Serialize)]
struct FolderStructure {
    /// 文件夹名称
    name: String,
    /// 文件夹路径
    path: String,
    /// 文件夹内子文件夹和文件列表
# 增强安全性
    children: Vec<FolderStructureItem>,
}

#[derive(Serialize)]
enum FolderStructureItem {
    /// 文件夹
    Folder(FolderStructure),
    /// 文件
    File(String),
}

#[get("/organize/<directory..>?<query>")]
fn organize(directory: PathBuf, query: Option<String>) -> Result<String, String> {
    let path = Path::new(&directory).to_path_buf();
    if !path.exists() {
        return Err("Directory does not exist".to_string());
    }
    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }

    let mut folder_structure = FolderStructure {
        name: path.file_name().unwrap_or_else(|| OsStr::new("").encode_wide().collect::<Vec<u16>>()).to_string_lossy().into_owned(),
        path: path.to_str().unwrap_or("").to_string(),
        children: Vec::new(),
# 添加错误处理
    };

    if let Err(e) = build_folder_structure(&mut folder_structure, &path) {
# 扩展功能模块
        return Err(e.to_string());
    }

    Ok(serde_json::to_string(&folder_structure).unwrap_or_else(|e| e.to_string()))
}

/// 递归构建文件夹结构
fn build_folder_structure(folder: &mut FolderStructure, path: &PathBuf) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
# 改进用户体验
        let path = entry.path();
        if path.is_dir() {
            let mut child_folder = FolderStructure {
                name: path.file_name().unwrap_or_else(|| OsStr::new("").encode_wide().collect::<Vec<u16>>()).to_string_lossy().into_owned(),
                path: path.to_str().unwrap_or("").to_string(),
                children: Vec::new(),
            };
# FIXME: 处理边界情况
            build_folder_structure(&mut child_folder, &path)?;
# 添加错误处理
            folder.children.push(FolderStructureItem::Folder(child_folder));
        } else {
            folder.children.push(FolderStructureItem::File(path.to_str().unwrap_or("
# 优化算法效率