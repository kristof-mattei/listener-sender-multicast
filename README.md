# Simple Sender and Listener for multicast in Rust

Ported from https://gist.github.com/hostilefork/f7cae3dc33e7416f2dd25a402857b6c6

Note that the listener binds to any address, and then joins the multicast group. This could be narrowed down to bind to the multicast group to filter out unicast traffic.

## Source Specific Multicast

To enable multicast with [Source Specific Multicast](https://en.wikipedia.org/wiki/Source-specific_multicast) modifications are required.

Note that this only applies when you use IGMPv3 AND your source is in the `232.0.0.0/8` range.

You need to do something like this in the listener (search for "`check the readme for SSM instructions`"):

```rust
fd.join_ssm_v4(
    &Ipv4Addr::new(172, 22, 92, 103),
    &cli.group,
    &Ipv4Addr::UNSPECIFIED,
)
.with_note(|| "setsockopt")?;
```

Credits: https://gist.github.com/hostilefork/f7cae3dc33e7416f2dd25a402857b6c6?permalink_comment_id=5065303#gistcomment-5065303

or force igmpv2:

<!-- prettier-ignore-start -->
```bash
echo "2" > /proc/sys/net/ipv4/conf/<your_interface>/force_igmp_version
```
<!-- prettier-ignore-end -->

Credits: https://gist.github.com/hostilefork/f7cae3dc33e7416f2dd25a402857b6c6?permalink_comment_id=5124051#gistcomment-5124051

## License

MIT, see [LICENSE](./LICENSE)

`SPDX-License-Identifier: MIT`
