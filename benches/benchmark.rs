#![feature(test)]
use casbin::prelude::*;

extern crate test;

use test::Bencher;

fn await_future<F, T>(future: F) -> T
where
    F: std::future::Future<Output = T>,
{
    #[cfg(feature = "runtime-async-std")]
    {
        async_std::task::block_on(future)
    }

    #[cfg(feature = "runtime-tokio")]
    {
        tokio::runtime::Runtime::new().unwrap().block_on(future)
    }
}

// To save a new baseline to compare against run
// `cargo bench > before`
// on the master branch.
//
// Then install cargo-benchcmp
// `cargo install cargo-benchcmp --force`
//
// Then to compare your changes switch to your branch and run
// `cargo bench > after`
//
// And compare with
// cargo benchcmp before after

fn raw_enforce(r: [&str; 3]) -> bool {
    let policies = [["alice", "data1", "read"], ["bob", "data2", "write"]];
    for policy in &policies {
        if policy == &r {
            return true;
        }
    }
    return false;
}

#[bench]
fn b_benchmark_raw(b: &mut Bencher) {
    b.iter(|| {
        raw_enforce(["alice", "data1", "read"]);
    });
}

#[bench]
fn b_benchmark_basic_model(b: &mut Bencher) {
    let mut e = await_future(Enforcer::new(
        "examples/basic_model.conf",
        "examples/basic_policy.csv",
    ))
    .unwrap();

    b.iter(|| {
        await_future(e.enforce(&["alice", "data1", "read"])).unwrap();
    });
}

#[bench]
fn b_benmark_cached_basic_model(b: &mut Bencher) {
    let mut e = await_future(CachedEnforcer::new(
        "examples/basic_model.conf",
        "examples/basic_policy.csv",
    ))
    .unwrap();

    b.iter(|| {
        await_future(e.enforce(&["alice", "data1", "read"])).unwrap();
    });
}

#[bench]
fn b_benchmark_rbac_model(b: &mut Bencher) {
    let mut e = await_future(Enforcer::new(
        "examples/rbac_model.conf",
        "examples/rbac_policy.csv",
    ))
    .unwrap();

    b.iter(|| {
        await_future(e.enforce(&["alice", "data2", "read"])).unwrap();
    });
}

#[bench]
fn b_benchmark_cached_rbac_model(b: &mut Bencher) {
    let mut e = await_future(CachedEnforcer::new(
        "examples/rbac_model.conf",
        "examples/rbac_policy.csv",
    ))
    .unwrap();

    b.iter(|| {
        await_future(e.enforce(&["alice", "data2", "read"])).unwrap();
    });
}

#[bench]
fn b_benchmark_rbac_model_small(b: &mut Bencher) {
    let mut e = await_future(Enforcer::new(
        "examples/rbac_model.conf",
        None as Option<&str>,
    ))
    .unwrap();

    e.enable_auto_build_role_links(false);

    // 100 roles, 10 resources.
    for i in 0..100 {
        await_future(e.add_policy(vec![
            format!("group{}", i),
            format!("data{}", i / 10),
            "read".to_owned(),
        ]))
        .unwrap();
    }

    // 1000 users.
    for i in 0..1000 {
        await_future(e.add_grouping_policy(vec![format!("user{}", i), format!("group{}", i / 10)]))
            .unwrap();
    }

    e.build_role_links().unwrap();

    b.iter(|| await_future(e.enforce(&["user501", "data9", "read"])).unwrap());
}

#[bench]
fn b_benchmark_cached_rbac_model_small(b: &mut Bencher) {
    let mut e = await_future(CachedEnforcer::new(
        "examples/rbac_model.conf",
        None as Option<&str>,
    ))
    .unwrap();

    e.enable_auto_build_role_links(false);

    // 100 roles, 10 resources.
    for i in 0..100 {
        await_future(e.add_policy(vec![
            format!("group{}", i),
            format!("data{}", i / 10),
            "read".to_owned(),
        ]))
        .unwrap();
    }

    // 1000 users.
    for i in 0..1000 {
        await_future(e.add_grouping_policy(vec![format!("user{}", i), format!("group{}", i / 10)]))
            .unwrap();
    }

    e.build_role_links().unwrap();

    b.iter(|| await_future(e.enforce(&["user501", "data9", "read"])).unwrap());
}

#[bench]
fn b_benchmark_rbac_model_medium(b: &mut Bencher) {
    let mut e = await_future(Enforcer::new(
        "examples/rbac_model.conf",
        None as Option<&str>,
    ))
    .unwrap();

    e.enable_auto_build_role_links(false);

    // 1000 roles, 100 resources.
    for i in 0..1000 {
        await_future(e.add_policy(vec![
            format!("group{}", i),
            format!("data{}", i / 10),
            "read".to_owned(),
        ]))
        .unwrap();
    }

    // 10000 users.
    for i in 0..10000 {
        await_future(e.add_grouping_policy(vec![format!("user{}", i), format!("group{}", i / 10)]))
            .unwrap();
    }

    e.build_role_links().unwrap();

    b.iter(|| await_future(e.enforce(&["user5001", "data15", "read"])).unwrap());
}

#[bench]
fn b_benchmark_cached_rbac_model_medium(b: &mut Bencher) {
    let mut e = await_future(CachedEnforcer::new(
        "examples/rbac_model.conf",
        None as Option<&str>,
    ))
    .unwrap();

    e.enable_auto_build_role_links(false);

    // 1000 roles, 100 resources.
    for i in 0..1000 {
        await_future(e.add_policy(vec![
            format!("group{}", i),
            format!("data{}", i / 10),
            "read".to_owned(),
        ]))
        .unwrap();
    }

    // 10000 users.
    for i in 0..10000 {
        await_future(e.add_grouping_policy(vec![format!("user{}", i), format!("group{}", i / 10)]))
            .unwrap();
    }

    e.build_role_links().unwrap();

    b.iter(|| await_future(e.enforce(&["user5001", "data150", "read"])).unwrap());
}

#[bench]
fn b_benchmark_rbac_model_large(b: &mut Bencher) {
    let mut e = await_future(Enforcer::new(
        "examples/rbac_model.conf",
        None as Option<&str>,
    ))
    .unwrap();

    e.enable_auto_build_role_links(false);

    // 10000 roles, 1000 resources.
    for i in 0..10000 {
        await_future(e.add_policy(vec![
            format!("group{}", i),
            format!("data{}", i / 10),
            "read".to_owned(),
        ]))
        .unwrap();
    }

    // 100000 users.
    for i in 0..100000 {
        await_future(e.add_grouping_policy(vec![format!("user{}", i), format!("group{}", i / 10)]))
            .unwrap();
    }

    e.build_role_links().unwrap();

    b.iter(|| await_future(e.enforce(&["user50001", "data1500", "read"])).unwrap());
}

#[bench]
fn b_benchmark_rbac_with_resource_roles(b: &mut Bencher) {
    let mut e = await_future(Enforcer::new(
        "examples/rbac_with_resource_roles_model.conf",
        "examples/rbac_with_resource_roles_policy.csv",
    ))
    .unwrap();

    b.iter(|| await_future(e.enforce(&["alice", "data1", "read"])).unwrap());
}

#[bench]
fn b_benchmark_cached_rbac_with_resource_roles(b: &mut Bencher) {
    let mut e = await_future(CachedEnforcer::new(
        "examples/rbac_with_resource_roles_model.conf",
        "examples/rbac_with_resource_roles_policy.csv",
    ))
    .unwrap();

    b.iter(|| await_future(e.enforce(&["alice", "data1", "read"])).unwrap());
}

#[bench]
fn b_benchmark_rbac_model_with_domains(b: &mut Bencher) {
    let mut e = await_future(Enforcer::new(
        "examples/rbac_with_domains_model.conf",
        "examples/rbac_with_domains_policy.csv",
    ))
    .unwrap();

    b.iter(|| await_future(e.enforce(&["alice", "domain1", "data1", "read"])).unwrap());
}

#[bench]
fn b_benchmark_cached_rbac_model_with_domains(b: &mut Bencher) {
    let mut e = await_future(CachedEnforcer::new(
        "examples/rbac_with_domains_model.conf",
        "examples/rbac_with_domains_policy.csv",
    ))
    .unwrap();

    b.iter(|| await_future(e.enforce(&["alice", "domain1", "data1", "read"])).unwrap());
}

#[bench]
fn b_benchmark_abac_model(b: &mut Bencher) {
    let mut e = await_future(Enforcer::new(
        "examples/abac_model.conf",
        None as Option<&str>,
    ))
    .unwrap();

    b.iter(|| await_future(e.enforce(&["alice", r#"{"Owner": "alice"}"#, "read"])).unwrap());
}

#[bench]
fn b_benchmark_cached_abac_model(b: &mut Bencher) {
    let mut e = await_future(CachedEnforcer::new(
        "examples/abac_model.conf",
        None as Option<&str>,
    ))
    .unwrap();

    b.iter(|| await_future(e.enforce(&["alice", r#"{"Owner": "alice"}"#, "read"])).unwrap());
}

#[bench]
fn b_benchmark_key_match(b: &mut Bencher) {
    let mut e = await_future(Enforcer::new(
        "examples/keymatch_model.conf",
        "examples/keymatch_policy.csv",
    ))
    .unwrap();

    b.iter(|| await_future(e.enforce(&["alice", "/alice_data/resource1", "GET"])).unwrap());
}

#[bench]
fn b_benchmark_cached_key_match(b: &mut Bencher) {
    let mut e = await_future(CachedEnforcer::new(
        "examples/keymatch_model.conf",
        "examples/keymatch_policy.csv",
    ))
    .unwrap();

    b.iter(|| await_future(e.enforce(&["alice", "/alice_data/resource1", "GET"])).unwrap());
}

#[bench]
fn b_benchmark_rbac_with_deny(b: &mut Bencher) {
    let mut e = await_future(Enforcer::new(
        "examples/rbac_with_deny_model.conf",
        "examples/rbac_with_deny_policy.csv",
    ))
    .unwrap();

    b.iter(|| await_future(e.enforce(&["alice", "data1", "read"])).unwrap());
}

#[bench]
fn b_benchmark_cached_rbac_with_deny(b: &mut Bencher) {
    let mut e = await_future(CachedEnforcer::new(
        "examples/rbac_with_deny_model.conf",
        "examples/rbac_with_deny_policy.csv",
    ))
    .unwrap();

    b.iter(|| await_future(e.enforce(&["alice", "data1", "read"])).unwrap());
}

#[bench]
fn b_benchmark_priority_model(b: &mut Bencher) {
    let mut e = await_future(Enforcer::new(
        "examples/priority_model.conf",
        "examples/priority_policy.csv",
    ))
    .unwrap();

    b.iter(|| await_future(e.enforce(&["alice", "data1", "read"])).unwrap());
}

#[bench]
fn b_benchmark_cached_priority_model(b: &mut Bencher) {
    let mut e = await_future(CachedEnforcer::new(
        "examples/priority_model.conf",
        "examples/priority_policy.csv",
    ))
    .unwrap();

    b.iter(|| await_future(e.enforce(&["alice", "data1", "read"])).unwrap());
}
