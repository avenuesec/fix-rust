
# fixcodegen

Generates the model, the parsing code and the builder code, from a standard fix dictionary xml file.


Usage example:

> cargo run --release -- ./specs/FIX42.xml ./template/ ./temp


For a simplified dictionary, use:

> cargo run --release -- ./specs/FIX42min.xml ./template/ ./temp


