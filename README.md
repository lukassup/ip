# ip command

iproute2 ip command for macOS and other platforms.


Currently implemented basic functionality for
- `link show`
- `address show`


```console
cargo run -- link show
1: lo: <UP,LOOPBACK,RUNNING,LOWER_UP>
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00

2: enp5s0: <UP,BROADCAST,RUNNING,MULTICAST,LOWER_UP>
    link/ether aa:bb:cc:00:11:22 brd ff:ff:ff:ff:ff:ff

3: wlp6s0: <BROADCAST,MULTICAST>
    link/ether bb:cc:dd:11:22:33 brd ff:ff:ff:ff:ff:ff

4: virbr0: <UP,BROADCAST,MULTICAST>
    link/ether cc:dd:ee:22:33:44 brd ff:ff:ff:ff:ff:ff

5: docker0: <UP,BROADCAST,MULTICAST>
    link/ether 99:88:77:66:55:44 brd ff:ff:ff:ff:ff:ff
```

```console
cargo run -- address show
1: lo: <UP,LOOPBACK,RUNNING,LOWER_UP>
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8
    inet6 ::1/128

2: enp5s0: <UP,BROADCAST,RUNNING,MULTICAST,LOWER_UP>
    link/ether aa:bb:cc:00:11:22 brd ff:ff:ff:ff:ff:ff
    inet 192.168.1.1/24 brd 192.168.88.255
    inet6 fe80::1010:2020:3030:4040/64

3: wlp6s0: <BROADCAST,MULTICAST>
    link/ether bb:cc:dd:11:22:33 brd ff:ff:ff:ff:ff:ff

4: virbr0: <UP,BROADCAST,MULTICAST>
    link/ether cc:dd:ee:22:33:44 brd ff:ff:ff:ff:ff:ff
    inet 192.168.122.1/24 brd 192.168.122.255

5: docker0: <UP,BROADCAST,MULTICAST>
    link/ether 99:88:77:66:55:44 brd ff:ff:ff:ff:ff:ff
    inet 172.17.0.1/16 brd 172.17.255.255
```