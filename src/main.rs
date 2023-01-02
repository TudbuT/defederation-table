use std::{env::args, fs};

use microasync::sync;
use microasync_util::{get_current_runtime, QueuedRuntime};

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
    kind: BlockKind,
}

impl From<&str> for InstanceBlock {
    fn from(value: &str) -> Self {
        let mut value = value.split("||");
        Self {
            url: value.next().unwrap().into(),
            reason: value.next().unwrap().into(),
            kind: value.next().unwrap().into(),
        }
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
    }
</style>

<div class=a><table>
    <tr>
        <th>URL</th>
        <th>Reason</th>
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
    let (url, reason, kind) = (
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
    </tr>",
        url.replace("\\", "\\\\").replace("\'", "\\\'"),
        url,
        reason.replace("\\", "\\\\").replace("\'", "\\\'"),
        reason,
        kind.replace("\\", "\\\\").replace("\'", "\\\'"),
        kind,
    );
}
