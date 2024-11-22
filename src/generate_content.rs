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

    let needs_math_lib = if sources.contains("math.h") { " -lm" } else { "" };
    format!(
        r#"
# Makefile for compiling and running {target}

CC = {compiler}              # The compiler ({compiler})
CFLAGS = -Wall {flags}        # Compiler flags
TARGET = {target}             # The output program name
SRC = {sources}               # The source files
OBJECTS = {objects}           # The object files

all: $(TARGET)
{tab}./$(TARGET)

$(TARGET): $(OBJECTS)
{tab}$(CC) $(CFLAGS) $(OBJECTS) -o $(TARGET){needs_math_lib}

%.o: %.c
{tab}$(CC) $(CFLAGS) -c $< -o $@

clean:
{tab}rm -f $(OBJECTS) $(TARGET)

.PHONY: all clean
    "#,
    tab = "\t",
    )
}
