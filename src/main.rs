use core::panic;
use std::{env::args, fs};

use microasync::sync;
use microasync_util::{get_current_runtime, QueuedRuntime};
use readformat::readf;

#[derive(Debug)]
#[allow(unused)]
enum BlockKind {
    RejectMedia,
    RejectReports,
    Silence,
    SilenceRejectMedia,
    SilenceRejectReports,
    Suspend,
}

impl From<&str> for BlockKind {
    fn from(value: &str) -> Self {
        if value == "RejectMedia" {
            return BlockKind::RejectMedia;
        }
        if value == "RejectReports" {
            return BlockKind::RejectReports;
        }
        if value == "Silence" {
            return BlockKind::Silence;
        }
        if value == "SilenceRejectMedia" {
            return BlockKind::SilenceRejectMedia;
        }
        if value == "SilenceRejectReports" {
            return BlockKind::SilenceRejectReports;
        }
        if value == "Suspend" {
            return BlockKind::Suspend;
        }
        panic!("Invalid BlockKind: {value}");
    }
}

struct InstanceBlock {
    url: String,
    reason: String,
    comment: String,
    kind: BlockKind,
}

impl From<&str> for InstanceBlock {
    fn from(value: &str) -> Self {
        if let Some(value) = readf("{}||{}||{}||{}", value) {
            return Self {
                url: value[0].clone(),
                reason: value[1].clone(),
                comment: value[2].clone(),
                kind: value[3].as_str().into(),
            };
        }
        if let Some(value) = readf("{}||{}||{}", value) {
            return Self {
                url: value[0].clone(),
                reason: value[1].clone(),
                comment: "-".into(),
                kind: value[2].as_str().into(),
            };
        }
        panic!("Invalid format: {value}");
    }
}

fn main() {
    println!(
        "{}",
        "
<style>
    @import url(\"https://fonts.googleapis.com/css?display=swap&amp;family=Inter:700,700italic,900,900italic%7CUbuntu:300,300italic,400,400italic\");

    table {
        width: 100%;
    }
    div.a {
        display: block;
        padding: 10px;
        border-radius: 10px;
        background-color: #2c2f35;
        overflow-x: scroll;
    }
    table, th, td {
        border: 1px solid #789;
        border-collapse: collapse;
        padding: 5px;
        color: #ddd;
        font-family: Ubuntu sans-serif;
        font-size: 15px;
    }
    td {
        cursor: pointer;
        user-select: none;
    }
    td:hover {
        background-color: #3c3f45;
    }
    td:active {
        background-color: #4c4f55;
    }
</style>

<div class=a><table>
    <tr>
        <th>URL</th>
        <th>Reason</th>
        <th>Comment</th>
        <th>Type</th>
    </tr>"
    );
    let mut runtime = QueuedRuntime::new();
    runtime.push(async {
        let file = fs::read_to_string(format!(
            "blocklist_{}.dbsv",
            args().skip(1).next().expect("please provide the category")
        ))
        .expect("file not found: blocklist_{arg}.dbsv!");

        for line in file.replace("\r\n", "\n").split("\n") {
            if line.is_empty() {
                continue;
            }
            get_current_runtime()
                .await
                .push(add(InstanceBlock::from(line)));
        }
    });
    sync(runtime);
    println!("</table></div>");
}

async fn add(block: InstanceBlock) {
    // maybe add dns check here later
    let (url, reason, comment, kind) = (
        block
            .url
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;"),
        block
            .reason
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;"),
        block
            .comment
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;"),
        format!("{:?}", block.kind)
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;"),
    );
    println!(
        "    <tr>
        <td onclick=\"navigator.clipboard.writeText('{}');\">{}</td>
        <td onclick=\"navigator.clipboard.writeText('{}');\">{}</td>
        <td onclick=\"navigator.clipboard.writeText('{}');\">{}</td>
        <td onclick=\"navigator.clipboard.writeText('{}');\">{}</td>
    </tr>",
        url.replace("\\", "\\\\").replace("\'", "\\\'"),
        url,
        reason.replace("\\", "\\\\").replace("\'", "\\\'"),
        reason,
        comment.replace("\\", "\\\\").replace("\'", "\\\'"),
        comment,
        kind.replace("\\", "\\\\").replace("\'", "\\\'"),
        kind,
    );
}
