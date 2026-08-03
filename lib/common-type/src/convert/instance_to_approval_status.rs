use crate::{ApprovalStatus, InstanceStatus};

impl From<InstanceStatus> for ApprovalStatus {
    fn from(status: InstanceStatus) -> Self {
        match status {
            InstanceStatus::Running => ApprovalStatus::Pending,
            InstanceStatus::Completed => ApprovalStatus::Approved,
            InstanceStatus::Rejected => ApprovalStatus::Rejected,
            InstanceStatus::Cancelled => ApprovalStatus::Cancelled,
        }
    }
}
