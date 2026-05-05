use petgraph::dot::{Dot, Config};
use petgraph::graph::{DiGraph, Graph, UnGraph};
use std::fmt::Display;
use std::process::Command;
use std::fs;
use std::fmt::Write;

/// 生成图的 DOT 文本，打印到终端，保存为 .dot 文件，并调用 Graphviz 渲染为 PNG 图片。
///
/// # 参数
/// - `graph`: 需要可视化的有向图（节点和边的数据类型需实现 `Display`）
/// - `dot_filename`: 保存的 .dot 文件名（例如 `"graph.dot"`）
/// - `png_filename`: 输出的 PNG 图片文件名（例如 `"graph.png"`）
///
/// # 注意
/// 要求系统中已安装 Graphviz 并将 `dot` 命令加入 PATH。
pub fn generate_dot_and_render_png<N, E>(
    graph: &Graph<N, E>,
    dot_filename: &str,
    png_filename: &str,
) where
    N: Display,
    E: Display,
{
    // 1. 生成 DOT 文本（默认显示边标签）
    let dot = Dot::with_config(graph, &[]);
    let dot_output = format!("{}", dot);

    // 2. 打印到终端
    println!("生成的 DOT 图文本：\n{}", dot_output);

    // 3. 将 DOT 文本保存到 .dot 文件
    fs::write(dot_filename, &dot_output).expect("Unable to write .dot file");

    // 4. 调用系统命令，使用 Graphviz 的 dot 工具将 .dot 文件渲染成 PNG 图片
    let output = Command::new("dot")
        .args(&["-Tpng", dot_filename, "-o", png_filename])
        .output()
        .expect("Failed to execute 'dot' command");

    if output.status.success() {
        println!("成功生成 {}", png_filename);
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        println!("渲染失败: {}", err);
    }
}

/// 生成无向图的 DOT 文本，打印到终端，保存为 .dot 文件，并调用 Graphviz 渲染为 PNG 图片。
///
/// # 参数
/// - `graph`: 需要可视化的无向图（节点和边的数据类型需实现 `Display`）
/// - `dot_filename`: 保存的 .dot 文件名（例如 `"undirected_graph.dot"`）
/// - `png_filename`: 输出的 PNG 图片文件名（例如 `"undirected_graph.png"`）
///
/// # 注意
/// 要求系统中已安装 Graphviz 并将 `dot` 命令加入 PATH。
pub fn generate_undirected_dot_and_render_png<N, E>(
    graph: &UnGraph<N, E>,
    dot_filename: &str,
    png_filename: &str,
) where
    N: Display,
    E: Display,
{
    // 1. 生成 DOT 文本（默认显示边标签，且 Graphviz 会识别为无向图）
    let dot = Dot::with_config(graph, &[Config::EdgeNoLabel]);
    let dot_output = format!("{}", dot);

    // 2. 打印到终端
    println!("生成的无向图 DOT 文本：\n{}", dot_output);

    // 3. 将 DOT 文本保存到 .dot 文件
    fs::write(dot_filename, &dot_output).expect("Unable to write .dot file");

    // 4. 调用系统命令，使用 Graphviz 的 dot 工具将 .dot 文件渲染成 PNG 图片
    let output = Command::new("dot")
        .args(&["-Tpng", dot_filename, "-o", png_filename])
        .output()
        .expect("Failed to execute 'dot' command");

    if output.status.success() {
        println!("成功生成 {}", png_filename);
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        println!("渲染失败: {}", err);
    }
}



use petgraph::graphmap::UnGraphMap;
use petgraph::graphmap::NodeTrait;  // 引入 NodeTrait


/// 生成无向图（GraphMap 版本）的 DOT 文本，打印到终端，保存为 .dot 文件，并调用 Graphviz 渲染为 PNG 图片。
///
/// # 参数
/// - `graph`: 需要可视化的无向图（节点类型需实现 `NodeTrait`，边类型需实现 `Display`）
/// - `dot_filename`: 保存的 .dot 文件名（例如 `"undirected_graphmap.dot"`）
/// - `png_filename`: 输出的 PNG 图片文件名（例如 `"undirected_graphmap.png"`）
///
/// # 注意
/// 要求系统中已安装 Graphviz 并将 `dot` 命令加入 PATH。
pub fn generate_undirected_graphmap_dot_and_render_png<N, E>(
    graph: &UnGraphMap<N, E>,
    dot_filename: &str,
    png_filename: &str,
) where
    N: NodeTrait + Display,   // NodeTrait 已包含 Copy + Ord + Hash + Debug
    E: Display,
{
    // 1. 生成 DOT 文本（默认显示边标签）
    let dot = Dot::with_config(graph, &[Config::EdgeNoLabel]);
    let dot_output = format!("{}", dot);

    // 2. 打印到终端
    println!("生成的 GraphMap 无向图 DOT 文本：\n{}", dot_output);

    // 3. 将 DOT 文本保存到 .dot 文件
    fs::write(dot_filename, &dot_output).expect("Unable to write .dot file");

    // 4. 调用系统命令，使用 Graphviz 的 dot 工具将 .dot 文件渲染成 PNG 图片
    let output = Command::new("dot")
        .args(&["-Tpng", dot_filename, "-o", png_filename])
        .output()
        .expect("Failed to execute 'dot' command");

    if output.status.success() {
        println!("成功生成 {}", png_filename);
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        println!("渲染失败: {}", err);
    }
}