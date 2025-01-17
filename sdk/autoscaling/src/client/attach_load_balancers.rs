// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AttachLoadBalancers`](crate::operation::attach_load_balancers::builders::AttachLoadBalancersFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`auto_scaling_group_name(impl ::std::convert::Into<String>)`](crate::operation::attach_load_balancers::builders::AttachLoadBalancersFluentBuilder::auto_scaling_group_name) / [`set_auto_scaling_group_name(Option<String>)`](crate::operation::attach_load_balancers::builders::AttachLoadBalancersFluentBuilder::set_auto_scaling_group_name): <p>The name of the Auto Scaling group.</p>
    ///   - [`load_balancer_names(Vec<String>)`](crate::operation::attach_load_balancers::builders::AttachLoadBalancersFluentBuilder::load_balancer_names) / [`set_load_balancer_names(Option<Vec<String>>)`](crate::operation::attach_load_balancers::builders::AttachLoadBalancersFluentBuilder::set_load_balancer_names): <p>The names of the load balancers. You can specify up to 10 load balancers.</p>
    /// - On success, responds with [`AttachLoadBalancersOutput`](crate::operation::attach_load_balancers::AttachLoadBalancersOutput)
    /// - On failure, responds with [`SdkError<AttachLoadBalancersError>`](crate::operation::attach_load_balancers::AttachLoadBalancersError)
    pub fn attach_load_balancers(&self) -> crate::operation::attach_load_balancers::builders::AttachLoadBalancersFluentBuilder {
        crate::operation::attach_load_balancers::builders::AttachLoadBalancersFluentBuilder::new(self.handle.clone())
    }
}
