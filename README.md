# may_exper

I want to play [May](https://github.com/Xudong-Huang/may). 

## Experiments


### OS Thread

```
$ cargo run --release --bin os_thread
```

My computer cannot spawn 1,000,000 threads (of course). 🤣 

### May

```
$ cargo run --release --bin may_go
```
Yes, it can run.


ps -auxw result is below:

```
vee      12677  105 35.8 3439056 2894400 pts/1 Sl+  21:03   0:23 target/release/may_go
```

Meaning, it used around 2.76 GB of RAM.
