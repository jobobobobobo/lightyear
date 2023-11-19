pub mod some_component {
    use bevy::prelude::Component;
    use serde::{Deserialize, Serialize};

    use lightyear_derive::{component_protocol, message_protocol};
    use lightyear_shared::{protocolize, Message};

    #[derive(Component, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
    pub struct Component1(pub u8);

    #[derive(Component, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
    pub struct Component2(pub u32);

    #[component_protocol(protocol = "MyProtocol")]
    pub enum MyComponentProtocol {
        #[prediction(rollback)]
        Component1(Component1),
        Component2(Component2),
    }

    #[derive(Message, Serialize, Deserialize, Debug, PartialEq, Clone)]
    pub struct Message1(pub u32);

    #[message_protocol(protocol = "MyProtocol")]
    pub enum MyMessageProtocol {
        Message1(Message1),
    }

    protocolize! {
        Self = MyProtocol,
        Message = MyMessageProtocol,
        Component = MyComponentProtocol,
    }
}

#[cfg(test)]
mod tests {
    use lightyear_shared::BitSerializable;
    use lightyear_shared::{ReadBuffer, ReadWordBuffer, WriteBuffer, WriteWordBuffer};

    use crate::some_component::MyComponentProtocol;

    use super::some_component::*;

    #[test]
    fn test_component_derive() -> anyhow::Result<()> {
        let component1: MyComponentProtocol = MyComponentProtocol::Component1(Component1(1));
        let mut writer = WriteWordBuffer::with_capacity(10);
        component1.encode(&mut writer)?;
        let bytes = writer.finish_write();

        let mut reader = ReadWordBuffer::start_read(bytes);
        let copy = MyComponentProtocol::decode(&mut reader)?;
        assert_eq!(component1, copy);

        Ok(())
    }
}
