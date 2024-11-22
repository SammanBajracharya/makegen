use crate::args::Args;

pub fn generate_makefile(args: &Args) -> String {
    let compiler = args.compiler.to_string();
    let flags = args.flags.clone();
    let sources = args.sources.join(" ");
    let objects = args
        .sources
        .iter()
        .map(|src| src.replace(".c", ".o"))
        .collect::<Vec<_>>()
        .join(" ");
    let target = args.output.clone();

    format!(
        r#"
CC = {compiler}
CFLAGS = {flags}
SOURCES = {sources}
OBJECTS = {objects}
TARGET = {target}

all: $(TARGET)

$(TARGET): $(OBJECTS)
\t$(CC) $(CFLAGS) -o $(TARGET) $(OBJECTS)

%.o: %.c
\t$(CC) $(CFLAGS) -c $< -o $@

clean:
\trm -f $(OBJECTS) $(TARGET)
    "#
    )
}
