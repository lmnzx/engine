use crate::cloud_provider::models::StorageDataTemplate;
use crate::cloud_provider::DeploymentTarget;
use crate::errors::EngineError;
use crate::models::container::Container;
use crate::models::types::{ToTeraContext, AWS};
use tera::Context as TeraContext;

impl ToTeraContext for Container<AWS> {
    fn to_tera_context(&self, target: &DeploymentTarget) -> Result<TeraContext, Box<EngineError>> {
        let mut context = self.default_tera_context(target);
        let storages = self
            .storages
            .iter()
            .map(|s| StorageDataTemplate {
                id: s.id.clone(),
                long_id: s.long_id,
                name: s.name.clone(),
                storage_type: s.storage_type.to_k8s_storage_class(),
                size_in_gib: s.size_in_gib,
                mount_point: s.mount_point.clone(),
                snapshot_retention_in_days: s.snapshot_retention_in_days,
            })
            .collect::<Vec<_>>();

        context.service.storages = storages;

        Ok(TeraContext::from_serialize(context).unwrap_or_default())
    }
}
