// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a VPC.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Vpc {
    /// <p>The primary IPv4 CIDR block for the VPC.</p>
    pub cidr_block: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the set of DHCP options you've associated with the VPC.</p>
    pub dhcp_options_id: ::std::option::Option<::std::string::String>,
    /// <p>The current state of the VPC.</p>
    pub state: ::std::option::Option<crate::types::VpcState>,
    /// <p>The ID of the VPC.</p>
    pub vpc_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Amazon Web Services account that owns the VPC.</p>
    pub owner_id: ::std::option::Option<::std::string::String>,
    /// <p>The allowed tenancy of instances launched into the VPC.</p>
    pub instance_tenancy: ::std::option::Option<crate::types::Tenancy>,
    /// <p>Information about the IPv6 CIDR blocks associated with the VPC.</p>
    pub ipv6_cidr_block_association_set: ::std::option::Option<::std::vec::Vec<crate::types::VpcIpv6CidrBlockAssociation>>,
    /// <p>Information about the IPv4 CIDR blocks associated with the VPC.</p>
    pub cidr_block_association_set: ::std::option::Option<::std::vec::Vec<crate::types::VpcCidrBlockAssociation>>,
    /// <p>Indicates whether the VPC is the default VPC.</p>
    pub is_default: ::std::option::Option<bool>,
    /// <p>Any tags assigned to the VPC.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl Vpc {
    /// <p>The primary IPv4 CIDR block for the VPC.</p>
    pub fn cidr_block(&self) -> ::std::option::Option<&str> {
        self.cidr_block.as_deref()
    }
    /// <p>The ID of the set of DHCP options you've associated with the VPC.</p>
    pub fn dhcp_options_id(&self) -> ::std::option::Option<&str> {
        self.dhcp_options_id.as_deref()
    }
    /// <p>The current state of the VPC.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::VpcState> {
        self.state.as_ref()
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>The ID of the Amazon Web Services account that owns the VPC.</p>
    pub fn owner_id(&self) -> ::std::option::Option<&str> {
        self.owner_id.as_deref()
    }
    /// <p>The allowed tenancy of instances launched into the VPC.</p>
    pub fn instance_tenancy(&self) -> ::std::option::Option<&crate::types::Tenancy> {
        self.instance_tenancy.as_ref()
    }
    /// <p>Information about the IPv6 CIDR blocks associated with the VPC.</p>
    pub fn ipv6_cidr_block_association_set(&self) -> ::std::option::Option<&[crate::types::VpcIpv6CidrBlockAssociation]> {
        self.ipv6_cidr_block_association_set.as_deref()
    }
    /// <p>Information about the IPv4 CIDR blocks associated with the VPC.</p>
    pub fn cidr_block_association_set(&self) -> ::std::option::Option<&[crate::types::VpcCidrBlockAssociation]> {
        self.cidr_block_association_set.as_deref()
    }
    /// <p>Indicates whether the VPC is the default VPC.</p>
    pub fn is_default(&self) -> ::std::option::Option<bool> {
        self.is_default
    }
    /// <p>Any tags assigned to the VPC.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl Vpc {
    /// Creates a new builder-style object to manufacture [`Vpc`](crate::types::Vpc).
    pub fn builder() -> crate::types::builders::VpcBuilder {
        crate::types::builders::VpcBuilder::default()
    }
}

/// A builder for [`Vpc`](crate::types::Vpc).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct VpcBuilder {
    pub(crate) cidr_block: ::std::option::Option<::std::string::String>,
    pub(crate) dhcp_options_id: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::VpcState>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
    pub(crate) owner_id: ::std::option::Option<::std::string::String>,
    pub(crate) instance_tenancy: ::std::option::Option<crate::types::Tenancy>,
    pub(crate) ipv6_cidr_block_association_set: ::std::option::Option<::std::vec::Vec<crate::types::VpcIpv6CidrBlockAssociation>>,
    pub(crate) cidr_block_association_set: ::std::option::Option<::std::vec::Vec<crate::types::VpcCidrBlockAssociation>>,
    pub(crate) is_default: ::std::option::Option<bool>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl VpcBuilder {
    /// <p>The primary IPv4 CIDR block for the VPC.</p>
    pub fn cidr_block(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cidr_block = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The primary IPv4 CIDR block for the VPC.</p>
    pub fn set_cidr_block(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cidr_block = input;
        self
    }
    /// <p>The primary IPv4 CIDR block for the VPC.</p>
    pub fn get_cidr_block(&self) -> &::std::option::Option<::std::string::String> {
        &self.cidr_block
    }
    /// <p>The ID of the set of DHCP options you've associated with the VPC.</p>
    pub fn dhcp_options_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.dhcp_options_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the set of DHCP options you've associated with the VPC.</p>
    pub fn set_dhcp_options_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.dhcp_options_id = input;
        self
    }
    /// <p>The ID of the set of DHCP options you've associated with the VPC.</p>
    pub fn get_dhcp_options_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.dhcp_options_id
    }
    /// <p>The current state of the VPC.</p>
    pub fn state(mut self, input: crate::types::VpcState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current state of the VPC.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::VpcState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The current state of the VPC.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::VpcState> {
        &self.state
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn get_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpc_id
    }
    /// <p>The ID of the Amazon Web Services account that owns the VPC.</p>
    pub fn owner_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the VPC.</p>
    pub fn set_owner_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the VPC.</p>
    pub fn get_owner_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.owner_id
    }
    /// <p>The allowed tenancy of instances launched into the VPC.</p>
    pub fn instance_tenancy(mut self, input: crate::types::Tenancy) -> Self {
        self.instance_tenancy = ::std::option::Option::Some(input);
        self
    }
    /// <p>The allowed tenancy of instances launched into the VPC.</p>
    pub fn set_instance_tenancy(mut self, input: ::std::option::Option<crate::types::Tenancy>) -> Self {
        self.instance_tenancy = input;
        self
    }
    /// <p>The allowed tenancy of instances launched into the VPC.</p>
    pub fn get_instance_tenancy(&self) -> &::std::option::Option<crate::types::Tenancy> {
        &self.instance_tenancy
    }
    /// Appends an item to `ipv6_cidr_block_association_set`.
    ///
    /// To override the contents of this collection use [`set_ipv6_cidr_block_association_set`](Self::set_ipv6_cidr_block_association_set).
    ///
    /// <p>Information about the IPv6 CIDR blocks associated with the VPC.</p>
    pub fn ipv6_cidr_block_association_set(mut self, input: crate::types::VpcIpv6CidrBlockAssociation) -> Self {
        let mut v = self.ipv6_cidr_block_association_set.unwrap_or_default();
        v.push(input);
        self.ipv6_cidr_block_association_set = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the IPv6 CIDR blocks associated with the VPC.</p>
    pub fn set_ipv6_cidr_block_association_set(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::VpcIpv6CidrBlockAssociation>>,
    ) -> Self {
        self.ipv6_cidr_block_association_set = input;
        self
    }
    /// <p>Information about the IPv6 CIDR blocks associated with the VPC.</p>
    pub fn get_ipv6_cidr_block_association_set(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::VpcIpv6CidrBlockAssociation>> {
        &self.ipv6_cidr_block_association_set
    }
    /// Appends an item to `cidr_block_association_set`.
    ///
    /// To override the contents of this collection use [`set_cidr_block_association_set`](Self::set_cidr_block_association_set).
    ///
    /// <p>Information about the IPv4 CIDR blocks associated with the VPC.</p>
    pub fn cidr_block_association_set(mut self, input: crate::types::VpcCidrBlockAssociation) -> Self {
        let mut v = self.cidr_block_association_set.unwrap_or_default();
        v.push(input);
        self.cidr_block_association_set = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the IPv4 CIDR blocks associated with the VPC.</p>
    pub fn set_cidr_block_association_set(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::VpcCidrBlockAssociation>>) -> Self {
        self.cidr_block_association_set = input;
        self
    }
    /// <p>Information about the IPv4 CIDR blocks associated with the VPC.</p>
    pub fn get_cidr_block_association_set(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::VpcCidrBlockAssociation>> {
        &self.cidr_block_association_set
    }
    /// <p>Indicates whether the VPC is the default VPC.</p>
    pub fn is_default(mut self, input: bool) -> Self {
        self.is_default = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the VPC is the default VPC.</p>
    pub fn set_is_default(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_default = input;
        self
    }
    /// <p>Indicates whether the VPC is the default VPC.</p>
    pub fn get_is_default(&self) -> &::std::option::Option<bool> {
        &self.is_default
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags assigned to the VPC.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Any tags assigned to the VPC.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>Any tags assigned to the VPC.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`Vpc`](crate::types::Vpc).
    pub fn build(self) -> crate::types::Vpc {
        crate::types::Vpc {
            cidr_block: self.cidr_block,
            dhcp_options_id: self.dhcp_options_id,
            state: self.state,
            vpc_id: self.vpc_id,
            owner_id: self.owner_id,
            instance_tenancy: self.instance_tenancy,
            ipv6_cidr_block_association_set: self.ipv6_cidr_block_association_set,
            cidr_block_association_set: self.cidr_block_association_set,
            is_default: self.is_default,
            tags: self.tags,
        }
    }
}
