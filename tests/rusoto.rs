use rusoto_core::Region;
use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, CreateLogGroupRequest, CreateLogStreamRequest,
    DescribeLogGroupsRequest, DescribeLogStreamsRequest, DescribeLogStreamsError,
    GetLogEventsRequest, InputLogEvent, LogGroup, PutLogEventsRequest,
};
use std::default::Default;

#[test]
fn describe_group() {
    let addr = start_server();
    let client = client(addr);

    let req = CreateLogGroupRequest {
        log_group_name: "test-group".into(),
        ..Default::default()
    };

    client.create_log_group(req).sync().unwrap();

    let mut desc_groups_req = DescribeLogGroupsRequest::default();
    desc_groups_req.log_group_name_prefix = "test-group".to_string().into();

    let res = client.describe_log_groups(desc_groups_req).sync().unwrap();

    let groups = res.log_groups.unwrap();
    assert_eq!(
        groups,
        vec![LogGroup {
            log_group_name: Some("test-group".into()),
            ..Default::default()
        }]
    );
}

#[test]
fn group_not_found() {
    let addr = start_server();
    let client = client(addr);

    let mut desc_streams_req = DescribeLogStreamsRequest::default();

    desc_streams_req.log_group_name = "non-existant-group".to_string();
    client
        .describe_log_streams(desc_streams_req)
        .sync()
        .unwrap_err();
}

#[test]
fn group_found() {
    let addr = start_server();
    let client = client(addr);

    let req = CreateLogGroupRequest {
        log_group_name: "test-group".into(),
        ..Default::default()
    };

    client.create_log_group(req).sync().unwrap();

    let mut desc_streams_req = DescribeLogStreamsRequest::default();
    desc_streams_req.log_group_name = "test-group".to_string();
    let res = client
        .describe_log_streams(desc_streams_req)
        .sync()
        .unwrap();

    let streams = res.log_streams.unwrap();
    assert_eq!(streams, vec![]);
}

#[test]
fn stream_found() {
    let addr = start_server();
    let client = client(addr);

    let group_name = "test-group".to_string();
    let req = CreateLogGroupRequest {
        log_group_name: group_name.clone(),
        ..Default::default()
    };

    client.create_log_group(req).sync().unwrap();

    let req = CreateLogStreamRequest {
        log_group_name: group_name.clone(),
        log_stream_name: "test-log-stream".into(),
    };

    client.create_log_stream(req).sync().unwrap();

    let desc_streams_req = DescribeLogStreamsRequest {
        log_group_name: group_name,
        ..Default::default()
    };

    let res = client
        .describe_log_streams(desc_streams_req)
        .sync()
        .unwrap();

    let streams = res.log_streams.unwrap();
    let stream = streams.into_iter().next().unwrap();
    let stream_name = stream.log_stream_name.unwrap();

    assert_eq!(stream_name, "test-log-stream".to_string());
}

#[test]
fn stream_failure() {
    let addr = start_server();
    let client = client(addr);

    let desc_streams_req = DescribeLogStreamsRequest {
        log_group_name: "ServiceUnavailable".into(),
        ..Default::default()
    };

    let res = client
        .describe_log_streams(desc_streams_req)
        .sync()
        .unwrap_err();
    match res {
      DescribeLogStreamsError::ServiceUnavailable(_) => (),
      x => panic!("{:?}", x)
    }
}

#[test]
fn create_group() {
    let addr = start_server();
    let client = client(addr);

    let req = CreateLogGroupRequest {
        log_group_name: "test-group-1".into(),
        ..Default::default()
    };

    client.create_log_group(req).sync().unwrap();
}

#[test]
fn create_stream() {
    let addr = start_server();
    let client = client(addr);

    let group_name = "test-group-1".to_string();
    let req = CreateLogGroupRequest {
        log_group_name: group_name.clone(),
        ..Default::default()
    };

    client.create_log_group(req).sync().unwrap();

    let req = CreateLogStreamRequest {
        log_group_name: group_name,
        log_stream_name: "test-log-stream".into(),
    };

    client.create_log_stream(req).sync().unwrap();
}

#[test]
fn put_logs_empty() {
    let addr = start_server();
    let client = client(addr);

    let group_name = "test-group".to_string();
    let req = CreateLogGroupRequest {
        log_group_name: group_name.clone(),
        ..Default::default()
    };

    client.create_log_group(req).sync().unwrap();

    let req = CreateLogStreamRequest {
        log_group_name: group_name.clone(),
        log_stream_name: "test-log-stream".into(),
    };

    client.create_log_stream(req).sync().unwrap();

    let desc_streams_req = DescribeLogStreamsRequest {
        log_group_name: group_name.clone(),
        ..Default::default()
    };

    let res = client
        .describe_log_streams(desc_streams_req)
        .sync()
        .unwrap();

    let streams = res.log_streams.unwrap();
    let stream = streams.into_iter().next().unwrap();
    let token = stream.upload_sequence_token.clone();
    let stream_name = stream.log_stream_name.unwrap();

    assert_eq!(stream_name, "test-log-stream".to_string());

    let req = PutLogEventsRequest {
        log_events: Vec::new(),
        log_group_name: group_name,
        log_stream_name: "test-log-stream".to_string(),
        sequence_token: token,
    };
    client.put_log_events(req).sync().unwrap();
}

#[test]
fn put_logs_non_empty() {
    let addr = start_server();
    let client = client(addr);

    let group_name = "test-group".to_string();
    let req = CreateLogGroupRequest {
        log_group_name: group_name.clone(),
        ..Default::default()
    };

    client.create_log_group(req).sync().unwrap();

    let req = CreateLogStreamRequest {
        log_group_name: group_name.clone(),
        log_stream_name: "test-log-stream".into(),
    };

    client.create_log_stream(req).sync().unwrap();

    let desc_streams_req = DescribeLogStreamsRequest {
        log_group_name: group_name.clone(),
        ..Default::default()
    };

    let res = client
        .describe_log_streams(desc_streams_req)
        .sync()
        .unwrap();

    let streams = res.log_streams.unwrap();
    let stream = streams.into_iter().next().unwrap();
    let token = stream.upload_sequence_token.clone();
    let stream_name = stream.log_stream_name.unwrap();

    assert_eq!(stream_name, "test-log-stream".to_string());

    let logs = vec![InputLogEvent {
        message: "hello world".into(),
        timestamp: chrono::Utc::now().timestamp_millis(),
    }];

    let req = PutLogEventsRequest {
        log_events: logs,
        log_group_name: group_name,
        log_stream_name: "test-log-stream".to_string(),
        sequence_token: token,
    };
    client.put_log_events(req).sync().unwrap();
}

#[test]
fn get_logs_empty() {
    let addr = start_server();
    let client = client(addr);

    let group_name = "test-group".to_string();
    let req = CreateLogGroupRequest {
        log_group_name: group_name.clone(),
        ..Default::default()
    };

    client.create_log_group(req).sync().unwrap();

    let req = CreateLogStreamRequest {
        log_group_name: group_name.clone(),
        log_stream_name: "test-log-stream".into(),
    };

    client.create_log_stream(req).sync().unwrap();

    let desc_streams_req = DescribeLogStreamsRequest {
        log_group_name: group_name.clone(),
        ..Default::default()
    };

    let res = client
        .describe_log_streams(desc_streams_req)
        .sync()
        .unwrap();

    let streams = res.log_streams.unwrap();
    let stream = streams.into_iter().next().unwrap();
    let token = stream.upload_sequence_token.clone();
    let stream_name = stream.log_stream_name.unwrap();

    assert_eq!(stream_name, "test-log-stream".to_string());

    let req = PutLogEventsRequest {
        log_events: Vec::new(),
        log_group_name: group_name.clone(),
        log_stream_name: "test-log-stream".to_string(),
        sequence_token: token,
    };
    client.put_log_events(req).sync().unwrap();

    let req = GetLogEventsRequest {
        log_stream_name: "test-log-stream".to_string(),
        log_group_name: group_name.clone(),
        ..Default::default()
    };

    let res = client.get_log_events(req).sync().unwrap();

    let events = res.events.unwrap();

    assert!(events.is_empty());
}

#[test]
fn get_logs_non_empty() {
    let addr = start_server();
    let client = client(addr);

    let group_name = "test-group".to_string();
    let req = CreateLogGroupRequest {
        log_group_name: group_name.clone(),
        ..Default::default()
    };

    client.create_log_group(req).sync().unwrap();

    let req = CreateLogStreamRequest {
        log_group_name: group_name.clone(),
        log_stream_name: "test-log-stream".into(),
    };

    client.create_log_stream(req).sync().unwrap();

    let desc_streams_req = DescribeLogStreamsRequest {
        log_group_name: group_name.clone(),
        ..Default::default()
    };

    let res = client
        .describe_log_streams(desc_streams_req)
        .sync()
        .unwrap();

    let streams = res.log_streams.unwrap();
    let stream = streams.into_iter().next().unwrap();
    let token = stream.upload_sequence_token.clone();
    let stream_name = stream.log_stream_name.unwrap();

    assert_eq!(stream_name, "test-log-stream".to_string());

    let logs = vec![InputLogEvent {
        message: "hello world".into(),
        timestamp: chrono::Utc::now().timestamp_millis(),
    }];

    let req = PutLogEventsRequest {
        log_events: logs,
        log_group_name: group_name.clone(),
        log_stream_name: "test-log-stream".to_string(),
        sequence_token: token,
    };
    client.put_log_events(req).sync().unwrap();

    let req = GetLogEventsRequest {
        log_stream_name: "test-log-stream".to_string(),
        log_group_name: group_name.clone(),
        ..Default::default()
    };

    let res = client.get_log_events(req).sync().unwrap();

    let events = res.events.unwrap();
    let event = events.into_iter().next().unwrap();
    let message = event.message.unwrap();

    assert_eq!(message, "hello world".to_string());
}

fn client(addr: SocketAddr) -> impl CloudWatchLogs {
    let endpoint = format!("http://localhost:{}", addr.port());
    let region = Region::Custom {
        name: "mockwatchlogs".into(),
        endpoint,
    };

    CloudWatchLogsClient::new(region)
}

fn start_server() -> SocketAddr {
    use mockwatchlogs::serve;
    let addr = next_addr();

    std::thread::spawn(move || {
        use tokio::runtime::current_thread;
        let serve = serve(addr);

        current_thread::run(serve);
    });

    std::thread::sleep(std::time::Duration::from_millis(100));

    addr
}

use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
static NEXT_PORT: AtomicUsize = AtomicUsize::new(1234);
pub fn next_addr() -> SocketAddr {
    use std::net::{IpAddr, Ipv4Addr};

    let port = NEXT_PORT.fetch_add(1, Ordering::AcqRel) as u16;
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
}
