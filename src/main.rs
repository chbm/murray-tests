
#![allow(dead_code)]
#![allow(unused)]

#[cfg(test)]
mod tests {
    use std::vec;
    use murray::actor;
    use tokio::sync::{mpsc, oneshot};
use tokio_test::{
    assert_err, assert_ok, assert_pending, assert_ready, assert_ready_err, assert_ready_ok,
};

    use String as TypeC;

    actor! {
        Unit
    }

    #[tokio::test]
    async fn test_unit_actor() {
        let u = UnitActor{}.start();

        // nothing happens here. can't instantiate a message
    }

#[derive(Debug)]
    pub struct FooMsg {
        a: u64,
        c: mpsc::Sender<u64>
    }

    actor! {
        Foo,
        Messages: {
            Msg1,
            Msg2 {a: u64, c: mpsc::Sender<u64>},
            Msg3 FooMsg,
        }
    }

    impl FooActor {
        async fn handle_msg1(&self, state: &mut FooActorState) {
            // nothing
        }
        async fn handle_msg2(&self, state: &mut FooActorState, payload: FooActorMessagesMsg2) {
            payload.c.send(payload.a).await;
        }
        async fn handle_msg3(&self, state: &mut FooActorState, payload: FooMsg) {
            payload.c.send(payload.a).await;
        }
    }

    #[tokio::test]
    async fn test_simple_actor() {
        eprintln!("!");
        let foo = FooActor{}.start();

        let (tx, mut rx) = mpsc::channel(2);

        assert_ok!(foo.send(FooActorMessages::Msg1).await);
        assert_ok!(foo.send(FooActorMessages::Msg2(FooActorMessagesMsg2{a: 1, c: tx.clone()})).await);
        assert_ok!(foo.send(FooActorMessages::Msg3(FooMsg{a: 2, c: tx})).await);

        assert_eq!(Some(1), rx.recv().await);
        assert_eq!(Some(2), rx.recv().await);
    }

    actor! {
        Sup,
        Messages: {
            M {s: String}
        }
    }

    impl SupActor {
        async fn handle_m(&self, state: &mut SupActorState, msg: SupActorMessagesM) {
        }
    }

    actor! {
        Bar,
        Options: {
            sup: Sup,
            id: String,
        },
        Messages: {
            A,
            B {
                x: bool,
            },
        },
        State: {
            foo: TypeC,
            bar: std::vec::Vec<String>,
        },
    }

    impl BarActor {
        async fn handle_a(&self, state: &mut BarActorState) {
            let b = if let Some(b) = state.bar.as_mut() { b } else { todo!()  };
            let c = if let Some(c) = state.sup_ch.as_mut() { c } else { todo!() };
            c.send(SupActorMessages::M(SupActorMessagesM{s: b[0].clone()})).await;
        }
        async fn handle_b(&self, state: &mut BarActorState, msg: BarActorMessagesB) {
            let b = if let Some(b) = state.bar.as_mut() { b } else { todo!()  };
            let c = if let Some(c) = state.sup_ch.as_mut() { c } else { todo!() };

            if msg.x {
                c.send(SupActorMessages::M(SupActorMessagesM{s: b[0].clone()})).await;
            } else {
                c.send(SupActorMessages::M(SupActorMessagesM{s: b[1].clone()})).await;
            }
        }
        fn init(&self, state: &mut BarActorState) {
            state.bar = Some(vec![state.id.clone(), String::from("no")]);
        }
    }

    #[tokio::test]
    async fn test_complex_actor() {
        let expected = String::from("BAR");

        let (suptx, mut suprx) = mpsc::channel::<SupActorMessages>(10);

        let bar = BarActor{}.start(suptx, &expected);
        
        assert_ok!(bar.send(BarActorMessages::A).await);
        assert_ok!(bar.send(BarActorMessages::B(BarActorMessagesB{x: true})).await);
        assert_ok!(bar.send(BarActorMessages::B(BarActorMessagesB{x: false})).await);


        if let Some(SupActorMessages::M(ret)) = suprx.recv().await {
            assert_eq!(ret.s, expected);
        } else {
            assert!(false);
        }
        if let Some(SupActorMessages::M(ret)) = suprx.recv().await {
            assert_eq!(ret.s, expected);
        } else {
            assert!(false);
        }
        if let Some(SupActorMessages::M(ret)) = suprx.recv().await {
            assert_eq!(ret.s, String::from("no"));
        } else {
            assert!(false);
        }
    }


}

fn main() {
    println!("Hello, world!");
}
