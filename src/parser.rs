use can_config_rs::{
    builder::{bus::BusBuilder, EnumBuilder, NetworkBuilder, NodeBuilder, StructBuilder},
    config::ObjectEntryAccess,
};

use can_config_rs::builder::TypeBuilder;

use can_config_rs::config::Type;

use crate::errors::{Error, Result};

pub fn parse_object_entry(
    oe_name: &str,
    oe_map: &yaml_rust::Yaml,
    node_builder: &mut NodeBuilder,
) -> Result<()> {
    let yaml_rust::Yaml::Hash(map) = oe_map else {
        return Err(Error::YamlInvalidType(format!(
            "object_entries have to be defined by mapps"
        )));
    };

    let yaml_rust::Yaml::String(type_name) = &oe_map["type"] else {
        return Err(Error::YamlInvalidType(format!(
            "types have to be defined as strings by their name"
        )));
    };

    let oe_builder = node_builder.create_object_entry(oe_name, &type_name);

    if map.contains_key(&yaml_rust::Yaml::String("description".to_owned())) {
        let yaml_rust::Yaml::String(description) = &oe_map["description"] else {
            return Err(Error::YamlInvalidType(format!(
                "descriptions have to be defined has strings"
            )));
        };
        oe_builder.add_description(&description);
    }

    if map.contains_key(&yaml_rust::Yaml::String("unit".to_owned())) {
        let yaml_rust::Yaml::String(unit) = &oe_map["unit"] else {
            return Err(Error::YamlInvalidType(format!(
                "unit has to be defined as a string"
            )));
        };
        oe_builder.add_unit(&unit);
    }

    if map.contains_key(&yaml_rust::Yaml::String("access".to_owned())) {
        let yaml_rust::Yaml::String(access) = &oe_map["access"] else {
            return Err(Error::YamlInvalidType(format!(
                "access has to be defined as a string"
            )));
        };
        let access = access.to_lowercase();
        if access == "const" {
            oe_builder.set_access(ObjectEntryAccess::Const);
        } else if access == "local" {
            oe_builder.set_access(ObjectEntryAccess::Local);
        } else if access == "global" {
            oe_builder.set_access(ObjectEntryAccess::Global);
        }
    }

    Ok(())
}

pub fn parse_tx_stream(
    stream_name: &str,
    stream_def: &yaml_rust::Yaml,
    node_builder: &mut NodeBuilder,
) -> Result<()> {
    let stream_builder = node_builder.create_stream(stream_name);

    let yaml_rust::Yaml::Hash(map) = stream_def else {
        return Err(Error::YamlInvalidType(format!(
            "streams have to be defined has maps"
        )));
    };

    if map.contains_key(&yaml_rust::Yaml::String("description".to_owned())) {
        let yaml_rust::Yaml::String(description) = &stream_def["description"] else {
            return Err(Error::YamlInvalidType(format!(
                "description has to be a string"
            )));
        };
        stream_builder.add_description(&description);
    }

    if map.contains_key(&yaml_rust::Yaml::String("mapping".to_owned())) {
        let yaml_rust::Yaml::Array(array) = &stream_def["mapping"] else {
            return Err(Error::YamlInvalidType(format!(
                "stream mappings have to be arrays"
            )));
        };
        for oe_name in array {
            let yaml_rust::Yaml::String(oe_name) = oe_name else {
                return Err(Error::YamlInvalidType(format!(
                    "stream mappings entries have to be names of object entries"
                )));
            };
            stream_builder.add_entry(oe_name);
        }
    }
    // TODO parse interval.

    Ok(())
}

pub fn parse_tx_command(
    command_name: &str,
    command_map: &yaml_rust::Yaml,
    node_builder: &mut NodeBuilder,
) -> Result<()> {
    let command_builder = node_builder.create_command(command_name, None);

    let yaml_rust::Yaml::Hash(map) = command_map else {
        return Err(Error::YamlInvalidType(format!(
            "commands have to be defined has maps"
        )));
    };

    if map.contains_key(&yaml_rust::Yaml::String("description".to_owned())) {
        let yaml_rust::Yaml::String(description) = &command_map["description"] else {
            return Err(Error::YamlInvalidType(format!(
                "description has to be a string"
            )));
        };
        command_builder.add_description(&description);
    }

    if map.contains_key(&yaml_rust::Yaml::String("arguments".to_owned())) {
        let yaml_rust::Yaml::Array(args) = &command_map["arguments"] else {
            return Err(Error::YamlInvalidType(format!(
                "the argument list of a command has to be a array"
            )));
        };
        for arg_map in args {
            let yaml_rust::Yaml::Hash(arg_map) = arg_map else {
                return Err(Error::YamlInvalidType(format!(
                    "command args have to be defined as \" <name> : <type> \" "
                )));
            };
            if arg_map.len() != 1 {
                return Err(Error::YamlInvalidType(format!(
                    "command args have to be defined as \" <name> : <type> \" "
                )));
            }
            let (name_yaml, type_yaml) = arg_map.iter().next().unwrap();
            let yaml_rust::Yaml::String(name) = name_yaml else {
                return Err(Error::YamlInvalidType(format!(
                    "command args have to be defined as \" <name> : <type> \" "
                )));
            };
            let yaml_rust::Yaml::String(ty) = type_yaml else {
                return Err(Error::YamlInvalidType(format!(
                    "command args have to be defined as \" <name> : <type> \" "
                )));
            };
            command_builder.add_argument(name, ty);
        }
    }

    if map.contains_key(&yaml_rust::Yaml::String("callee".to_owned())) {
        let yaml_rust::Yaml::Array(callees) = &command_map["callee"] else {
            return Err(Error::YamlInvalidType(format!(
                "the argument list of a command has to be a array"
            )));
        };
        for callee in callees {
            let yaml_rust::Yaml::String(callee_name) = callee else {
                return Err(Error::YamlInvalidType(format!(
                    "callees have to refered to by name (String)"
                )));
            };
            command_builder.add_callee(callee_name);
        }
    }

    Ok(())
}

pub fn parse_rx_stream(
    node_name: &str,
    stream_name: &str,
    stream_def: &yaml_rust::Yaml,
    node_builder: &mut NodeBuilder,
) -> Result<()> {
    let rx_stream_builder = node_builder.receive_stream(node_name, stream_name);

    //parse stream_def as oe mapping
    let yaml_rust::Yaml::Hash(map) = stream_def else {
        return Err(Error::YamlInvalidType(format!(
            "rx_streams have to be defined as maps of oe entries"
        )));
    };
    for (tx_oe_name, rx_oe_name) in map {
        let yaml_rust::Yaml::String(tx_oe_name) = tx_oe_name else {
            return Err(Error::YamlInvalidType(format!(
                "object entries have to be refered to by name in rx_stream definition"
            )));
        };
        let yaml_rust::Yaml::String(rx_oe_name) = rx_oe_name else {
            return Err(Error::YamlInvalidType(format!(
                "object entries have to be refered to by name in rx_stream definition"
            )));
        };
        rx_stream_builder.map(tx_oe_name, rx_oe_name);
    }
    Ok(())
}

pub fn parse_node(
    node_name: &str,
    node_map: &yaml_rust::Yaml,
    network_builder: &mut NetworkBuilder,
) -> Result<()> {
    let mut node_builder = network_builder.create_node(node_name);
    let yaml_rust::Yaml::Hash(map) = node_map else {
        return Err(Error::YamlInvalidType(format!(
            "nodes have to be defined has maps"
        )));
    };

    if map.contains_key(&yaml_rust::Yaml::String("description".to_owned())) {
        let yaml_rust::Yaml::String(description) = &node_map["description"] else {
            return Err(Error::YamlInvalidType(format!(
                "description has to be a string"
            )));
        };
        node_builder.add_description(&description);
    }
    if map.contains_key(&yaml_rust::Yaml::String("object_dictionary".to_owned())) {
        let yaml_rust::Yaml::Hash(od_map) = &node_map["object_dictionary"] else {
            return Err(Error::YamlInvalidType(format!(
                "object_dictionary has to be a map"
            )));
        };
        for (name, oe_map) in od_map {
            let yaml_rust::Yaml::String(name) = name else {
                return Err(Error::YamlInvalidType(format!(
                    "the name of a object_entry has to be a string"
                )));
            };
            parse_object_entry(name, oe_map, &mut node_builder)?;
        }
    }

    if map.contains_key(&yaml_rust::Yaml::String("tx_streams".to_owned())) {
        let yaml_rust::Yaml::Hash(streams_map) = &node_map["tx_streams"] else {
            return Err(Error::YamlInvalidType(format!(
                "tx_streams have to be defined has maps"
            )));
        };
        for (stream_name, stream_def) in streams_map {
            let yaml_rust::Yaml::String(stream_name) = stream_name else {
                return Err(Error::YamlInvalidType(format!(
                    "the name of a stream has to be a string"
                )));
            };
            parse_tx_stream(&stream_name, stream_def, &mut node_builder)?;
        }
    }

    if map.contains_key(&yaml_rust::Yaml::String("rx_streams".to_owned())) {
        let yaml_rust::Yaml::Hash(rx_node) = &node_map["rx_streams"] else {
            return Err(Error::YamlInvalidType(format!(
                "rx_streams have to be defined has maps"
            )));
        };
        for (node_name, tx_node_streams) in rx_node {
            let yaml_rust::Yaml::String(node_name) = node_name else {
                return Err(Error::YamlInvalidType(format!(
                    "rx_streams has to contains the names of the tx_nodes has strings"
                )));
            };
            let yaml_rust::Yaml::Hash(tx_node_streams) = tx_node_streams else {
                return Err(Error::YamlInvalidType(format!(
                    "rx_streams has to be a map of names of tx_nodes, which has to be a map of the tx_streams that are received"
                )));
            };
            for (stream_name, stream_def) in tx_node_streams {
                let yaml_rust::Yaml::String(stream_name) = stream_name else {
                    return Err(Error::YamlInvalidType(format!(
                        "stream names have to be defined as strings"
                    )));
                };
                parse_rx_stream(node_name, stream_name, stream_def, &mut node_builder)?;
            }
        }
    }

    if map.contains_key(&yaml_rust::Yaml::String("commands".to_owned())) {
        let yaml_rust::Yaml::Hash(commands) = &node_map["commands"] else {
            return Err(Error::YamlInvalidType(format!(
                "command lists have to be defined has maps"
            )));
        };
        for (command_name, command_def) in commands {
            let yaml_rust::Yaml::String(command_name) = command_name else {
                return Err(Error::YamlInvalidType(format!(
                    "the name of a command has to be a string"
                )));
            };
            parse_tx_command(command_name, command_def, &mut node_builder)?;
        }
    }
    Ok(())
}

pub fn parse_enum_type(enum_map: &yaml_rust::Yaml, enum_builder: &mut EnumBuilder) -> Result<()> {
    let yaml_rust::Yaml::Hash(enum_hash_map) = enum_map else {
        return Err(Error::YamlInvalidType(format!(
            "Enums have to be given as a map with variants"
        )));
    };
    for (variant_name, variant_value) in enum_hash_map {
        let yaml_rust::Yaml::String(variant_name) = variant_name else {
            return Err(Error::YamlInvalidType(format!(
                "enum variants must be string"
            )));
        };
        let value = match variant_value {
            yaml_rust::Yaml::Integer(value) => {
                if *value < 0 {
                    return Err(Error::YamlInvalidType(format!(
                        "enum values must be positive"
                    )));
                }
                Some(*value as u64)
            }
            yaml_rust::Yaml::Null => None,
            _ => {
                return Err(Error::YamlInvalidType(format!(
                    "enum variants must be string"
                )));
            }
        };
        enum_builder.add_entry(variant_name, value)?;
    }
    Ok(())
}

pub fn parse_struct_type(
    struct_map: &yaml_rust::Yaml,
    struct_builder: &mut StructBuilder,
) -> Result<()> {
    let yaml_rust::Yaml::Hash(struct_hash_map) = struct_map else {
        return Err(Error::YamlInvalidType(format!(
            "structs have to be given as a map with of attributes"
        )));
    };
    for (attribute_name, attribute_type) in struct_hash_map {
        let yaml_rust::Yaml::String(attribute_name) = attribute_name else {
            return Err(Error::YamlInvalidType(format!(
                "struct attributes have to be denoted by strings"
            )));
        };
        let yaml_rust::Yaml::String(attribute_type) = attribute_type else {
            return Err(Error::YamlInvalidType(format!(
                "struct attributes need to have a string encoded type"
            )));
        };
        struct_builder.add_attribute(attribute_name, attribute_type)?;
    }
    Ok(())
}

pub fn parse_bus(bus_map: &yaml_rust::Yaml, bus_builder: &mut BusBuilder) -> Result<()> {
    let yaml_rust::Yaml::Hash(bus_hash_map) = bus_map else {
        return Err(Error::YamlInvalidType(format!(
            "bus must be described by a map"
        )));
    };
    
    bus_hash_map.get(&yaml_rust::yaml::Yaml::String("baudrate".to_owned())).map(|yaml| {
        let yaml_rust::Yaml::Integer(baudrate) = yaml else {
            panic!("baudrate must be integer value");
        };
        bus_builder.baudrate(*baudrate as u32);
    });

    Ok(())
}

pub fn parse_top_level(
    yaml: &yaml_rust::yaml::Yaml,
    network_builder: &mut NetworkBuilder,
) -> Result<()> {
    let yaml_rust::Yaml::Hash(_) = yaml else {
        return Err(Error::YamlInvalidFormat(format!("")));
    };

    let yaml_rust::Yaml::Hash(nodes_map) = &yaml["nodes"] else {
        return Err(Error::YamlInvalidType(format!(
            "nodes have to be defined as a map"
        )));
    };
    for (name, node_def) in nodes_map {
        let yaml_rust::Yaml::String(name) = name else {
            return Err(Error::YamlInvalidType(format!(
                "name of a node has to be a string"
            )));
        };
        parse_node(name, node_def, network_builder)?;
    }

    let yaml_rust::Yaml::Hash(structs_map) = &yaml["struct_types"] else {
        return Err(Error::YamlInvalidType(format!(
            "structs must be given as a map"
        )));
    };
    for (name, struct_map) in structs_map {
        let yaml_rust::Yaml::String(struct_name) = name else {
            return Err(Error::YamlInvalidType(format!(
                "struct names must be primitive string keys"
            )));
        };

        parse_struct_type(struct_map, &mut network_builder.define_struct(struct_name))?;
    }

    let yaml_rust::Yaml::Hash(enums_map) = &yaml["enum_types"] else {
        return Err(Error::YamlInvalidType(format!(
            "enums must be given as a map"
        )));
    };
    for (name, enum_map) in enums_map {
        let yaml_rust::Yaml::String(enum_name) = name else {
            return Err(Error::YamlInvalidType(format!(
                "struct names must be primitive string keys"
            )));
        };

        parse_enum_type(enum_map, &mut network_builder.define_enum(enum_name))?;
    }

    let yaml_rust::Yaml::Hash(bus_map) = &yaml["buses"] else {
        return Err(Error::YamlInvalidType(format!(
            "buses must be given as a map"
        )));
    };
    for (name, bus_map) in bus_map {
        let yaml_rust::Yaml::String(bus_name) = name else {
            return Err(Error::YamlInvalidType(format!(
                "bus names must be primitive string keys"
            )));
        };

        parse_bus(bus_map, &mut network_builder.create_bus(bus_name, None))?;
    }

    Ok(())
}
