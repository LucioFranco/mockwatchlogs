use crate::streams::{Group, Stream};
use crate::types::*;
use crate::{Body, Context, Response};
use serde_json::{json, Value};

pub fn describe_streams(
    context: &mut Context,
    request: DescribeLogStreamsRequest,
) -> Result<Response, ServiceError> {
    let streams = if let Some(err) = to_service_error(&request.log_group_name) {
        return Err(err);
    } else if let Some(group) = context.groups.get(&request.log_group_name) {
        group.streams.clone()
    } else {
        return Err(ServiceError::NotFound("Group not found".into()));
    };

    let streams = if let Some(prefix) = request.log_stream_name_prefix {
        streams
            .into_iter()
            .filter(|stream| stream.name.starts_with(prefix.as_str()))
            .map(|e| e.into())
            .collect()
    } else {
        streams.into_iter().map(|e| e.into()).collect()
    };

    let res = DescribeLogStreamsResponse {
        log_streams: Some(streams),
        next_token: None,
    };

    let body = serde_json::to_vec(&res).unwrap();
    Ok(Response::new(Body::from(body)))
}

pub fn describe_groups(
    context: &mut Context,
    request: DescribeLogGroupsRequest,
) -> Result<Response, ServiceError> {
    if let Some(name) = &request.log_group_name_prefix {
        if let Some(group) = context.groups.get(name) {
            let group = LogGroup {
                log_group_name: Some(group.name.clone()),
                ..Default::default()
            };

            let response = DescribeLogGroupsResponse {
                log_groups: vec![group].into(),
                next_token: Some("token".into()),
            };

            let body = serde_json::to_vec(&response).unwrap();
            Ok(Response::new(Body::from(body)))
        } else {
            Err(ServiceError::NotFound(name.clone()))
        }
    } else {
        unreachable!()
    }
}

pub fn create_group(
    context: &mut Context,
    request: CreateLogGroupRequest,
) -> Result<Response, ServiceError> {
    if let None = context.groups.get(&request.log_group_name) {
        context.groups.insert(
            request.log_group_name.clone(),
            Group {
                name: request.log_group_name,
                ..Default::default()
            },
        );
        Ok(Response::new(Body::empty()))
    } else {
        Err(ServiceError::ResourceAlreadyExistsException)
    }
}

pub fn create_stream(
    context: &mut Context,
    request: CreateLogStreamRequest,
) -> Result<Response, ServiceError> {
    if let Some(ref mut group) = context.groups.get_mut(&request.log_group_name) {
        if let None = group
            .streams
            .iter()
            .find(|e| e.name == request.log_stream_name)
        {
            group.streams.push(Stream {
                name: request.log_stream_name.clone(),
                logs: Vec::new(),
            });
            Ok(Response::new(Body::empty()))
        } else {
            Err(ServiceError::ResourceAlreadyExistsException)
        }
    } else {
        Err(ServiceError::NotFound("Group not found".into()))
    }
}

pub fn put_logs(
    context: &mut Context,
    request: PutLogEventsRequest,
) -> Result<Response, ServiceError> {
    if let Some(ref mut group) = context.groups.get_mut(&request.log_group_name) {
        if let Some(stream) = group
            .streams
            .iter_mut()
            .find(|e| e.name == request.log_stream_name)
        {
            // TODO: validate logs?
            stream.logs.extend(request.log_events.clone());

            let res = PutLogEventsResponse {
                ..Default::default()
            };

            let body = serde_json::to_vec(&res).unwrap();
            Ok(Response::new(Body::from(body)))
        } else {
            Err(ServiceError::NotFound("Stream not found".into()))
        }
    } else {
        Err(ServiceError::NotFound("Group not found".into()))
    }
}

pub fn get_logs(
    context: &mut Context,
    request: GetLogEventsRequest,
) -> Result<Response, ServiceError> {
    if let Some(ref mut group) = context.groups.get_mut(&request.log_group_name) {
        if let Some(stream) = group
            .streams
            .iter()
            .find(|e| e.name == request.log_stream_name)
        {
            let logs = stream
                .logs
                .iter()
                .filter(|log| {
                    if let Some(start_time) = request.start_time {
                        log.timestamp >= start_time
                    } else {
                        true
                    }
                })
                .map(|l| OutputLogEvent {
                    message: Some(l.message.clone()),
                    timestamp: Some(l.timestamp),
                    ..Default::default()
                })
                .collect();

            let res = GetLogEventsResponse {
                events: Some(logs),
                ..Default::default()
            };

            let body = serde_json::to_vec(&res).unwrap();
            Ok(Response::new(Body::from(body)))
        } else {
            Err(ServiceError::NotFound("Stream not found".into()))
        }
    } else {
        Err(ServiceError::NotFound("Group not found".into()))
    }
}

pub enum ServiceError {
    ServiceUnavailable,
    NotFound(String),
    ResourceAlreadyExistsException,
}

fn to_service_error(e: &String) -> Option<ServiceError> {
    if e == "ServiceUnavailable" {
        Some(ServiceError::ServiceUnavailable)
    } else {
        None
    }
}

fn to_response(json: &Value) -> hyper::Response<hyper::Body> {
    let body = serde_json::to_vec(json).unwrap();

    hyper::Response::builder()
        .status(400)
        .body(hyper::Body::from(body))
        .unwrap()
}

impl From<ServiceError> for hyper::Response<hyper::Body> {
    fn from(e: ServiceError) -> Self {
        match e {
            ServiceError::ServiceUnavailable =>
                to_response(&json!({
                    "__type": "ServiceUnavailableException",
                    "message": "Gone fishing"
                })),
            ServiceError::NotFound(message) =>
                to_response(&json!({
                    "__type": "ResourceNotFoundException",
                    "message": message
                })),
            ServiceError::ResourceAlreadyExistsException =>
                to_response(&json!({
                    "__type": "ResourceAlreadyExistsException",
                    "message": "Resource not found"
                }))
        }
    }
}
