// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteIpam`](crate::operation::delete_ipam::builders::DeleteIpamFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::delete_ipam::builders::DeleteIpamFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_ipam::builders::DeleteIpamFluentBuilder::set_dry_run): <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`ipam_id(impl ::std::convert::Into<String>)`](crate::operation::delete_ipam::builders::DeleteIpamFluentBuilder::ipam_id) / [`set_ipam_id(Option<String>)`](crate::operation::delete_ipam::builders::DeleteIpamFluentBuilder::set_ipam_id): <p>The ID of the IPAM to delete.</p>
    ///   - [`cascade(bool)`](crate::operation::delete_ipam::builders::DeleteIpamFluentBuilder::cascade) / [`set_cascade(Option<bool>)`](crate::operation::delete_ipam::builders::DeleteIpamFluentBuilder::set_cascade): <p>Enables you to quickly delete an IPAM, private scopes, pools in private scopes, and any allocations in the pools in private scopes. You cannot delete the IPAM with this option if there is a pool in your public scope. If you use this option, IPAM does the following:</p>  <ul>   <li> <p>Deallocates any CIDRs allocated to VPC resources (such as VPCs) in pools in private scopes.</p> <note>     <p>No VPC resources are deleted as a result of enabling this option. The CIDR associated with the resource will no longer be allocated from an IPAM pool, but the CIDR itself will remain unchanged.</p>    </note> </li>   <li> <p>Deprovisions all IPv4 CIDRs provisioned to IPAM pools in private scopes.</p> </li>   <li> <p>Deletes all IPAM pools in private scopes.</p> </li>   <li> <p>Deletes all non-default private scopes in the IPAM.</p> </li>   <li> <p>Deletes the default public and private scopes and the IPAM.</p> </li>  </ul>
    /// - On success, responds with [`DeleteIpamOutput`](crate::operation::delete_ipam::DeleteIpamOutput) with field(s):
    ///   - [`ipam(Option<Ipam>)`](crate::operation::delete_ipam::DeleteIpamOutput::ipam): <p>Information about the results of the deletion.</p>
    /// - On failure, responds with [`SdkError<DeleteIpamError>`](crate::operation::delete_ipam::DeleteIpamError)
    pub fn delete_ipam(&self) -> crate::operation::delete_ipam::builders::DeleteIpamFluentBuilder {
        crate::operation::delete_ipam::builders::DeleteIpamFluentBuilder::new(self.handle.clone())
    }
}
