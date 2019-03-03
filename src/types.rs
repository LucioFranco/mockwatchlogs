// Types taken from rusoto_logs
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputLogEvent {
    /// <p>The raw event message.</p>
    #[serde(rename = "message")]
    pub message: String,
    /// <p>The time the event occurred, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeLogStreamsRequest {
    #[serde(rename = "descending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descending: Option<bool>,
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    #[serde(rename = "logStreamNamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefix: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "orderBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeLogStreamsResponse {
    /// <p>The log streams.</p>
    #[serde(rename = "logStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_streams: Option<Vec<LogStream>>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents a log stream, which is a sequence of log events from a single emitter of logs.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LogStream {
    /// <p>The Amazon Resource Name (ARN) of the log stream.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The creation time of the stream, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    /// <p>The time of the first event, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "firstEventTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_event_timestamp: Option<i64>,
    /// <p>The time of the most recent log event in the log stream in CloudWatch Logs. This number is expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. The <code>lastEventTime</code> value updates on an eventual consistency basis. It typically updates in less than an hour from ingestion, but may take longer in some rare situations.</p>
    #[serde(rename = "lastEventTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_event_timestamp: Option<i64>,
    /// <p>The ingestion time, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "lastIngestionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_ingestion_time: Option<i64>,
    /// <p>The name of the log stream.</p>
    #[serde(rename = "logStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    /// <p>The number of bytes stored.</p>
    #[serde(rename = "storedBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_bytes: Option<i64>,
    /// <p>The sequence token.</p>
    #[serde(rename = "uploadSequenceToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_sequence_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateLogGroupRequest {
    /// <p>The Amazon Resource Name (ARN) of the CMK to use when encrypting log data. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kms">Amazon Resource Names - AWS Key Management Service (AWS KMS)</a>.</p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The key-value pairs to use for the tags.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateLogStreamRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The name of the log stream.</p>
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutLogEventsRequest {
    /// <p>The log events.</p>
    #[serde(rename = "logEvents")]
    pub log_events: Vec<InputLogEvent>,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The name of the log stream.</p>
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
    /// <p>The sequence token obtained from the response of the previous <code>PutLogEvents</code> call. An upload in a newly created log stream does not require a sequence token. You can also get the sequence token using <a>DescribeLogStreams</a>. If you call <code>PutLogEvents</code> twice within a narrow time period using the same value for <code>sequenceToken</code>, both calls may be successful, or one may be rejected.</p>
    #[serde(rename = "sequenceToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutLogEventsResponse {
    /// <p>The next sequence token.</p>
    #[serde(rename = "nextSequenceToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_sequence_token: Option<String>,
    /// <p>The rejected events.</p>
    #[serde(rename = "rejectedLogEventsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_log_events_info: Option<RejectedLogEventsInfo>,
}

/// <p>Represents the rejected events.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RejectedLogEventsInfo {
    /// <p>The expired log events.</p>
    #[serde(rename = "expiredLogEventEndIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_log_event_end_index: Option<i64>,
    /// <p>The log events that are too new.</p>
    #[serde(rename = "tooNewLogEventStartIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub too_new_log_event_start_index: Option<i64>,
    /// <p>The log events that are too old.</p>
    #[serde(rename = "tooOldLogEventEndIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub too_old_log_event_end_index: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetLogEventsRequest {
    /// <p>The end of the time range, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a timestamp equal to or later than this time are not included.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// <p>The maximum number of log events returned. If you don't specify a value, the maximum is as many log events as can fit in a response size of 1 MB, up to 10,000 log events.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The name of the log stream.</p>
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the value is true, the earliest log events are returned first. If the value is false, the latest log events are returned first. The default value is false.</p>
    #[serde(rename = "startFromHead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_from_head: Option<bool>,
    /// <p>The start of the time range, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a timestamp equal to this time or later than this time are included. Events with a timestamp earlier than this time are not included.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLogEventsResponse {
    /// <p>The events.</p>
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<OutputLogEvent>>,
    /// <p>The token for the next set of items in the backward direction. The token expires after 24 hours. This token will never be null. If you have reached the end of the stream, it will return the same token you passed in.</p>
    #[serde(rename = "nextBackwardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_backward_token: Option<String>,
    /// <p>The token for the next set of items in the forward direction. The token expires after 24 hours. If you have reached the end of the stream, it will return the same token you passed in.</p>
    #[serde(rename = "nextForwardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_forward_token: Option<String>,
}

/// <p>Represents a log event.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct OutputLogEvent {
    /// <p>The time the event was ingested, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "ingestionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_time: Option<i64>,
    /// <p>The data contained in the log event.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The time the event occurred, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}
