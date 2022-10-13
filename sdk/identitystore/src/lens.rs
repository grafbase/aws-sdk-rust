// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_group_memberships_output_next_token(
    input: &crate::output::ListGroupMembershipsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_group_memberships_for_member_output_next_token(
    input: &crate::output::ListGroupMembershipsForMemberOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_groups_output_next_token(
    input: &crate::output::ListGroupsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_users_output_next_token(
    input: &crate::output::ListUsersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_group_memberships_output_group_memberships(
    input: crate::output::ListGroupMembershipsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::GroupMembership>> {
    let input = match input.group_memberships {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_group_memberships_for_member_output_group_memberships(
    input: crate::output::ListGroupMembershipsForMemberOutput,
) -> std::option::Option<std::vec::Vec<crate::model::GroupMembership>> {
    let input = match input.group_memberships {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_groups_output_groups(
    input: crate::output::ListGroupsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Group>> {
    let input = match input.groups {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_users_output_users(
    input: crate::output::ListUsersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::User>> {
    let input = match input.users {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
