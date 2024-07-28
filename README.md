# Vector Remap Language
This repo contains information about vector remap language.

---
<details>
  <summary> What is Vector Remap Language </summary>
- **Vector Remap Language (VRL)**
  - It is an expression-oriented language designed for transforming observability data (logs and metrics) in a safe and performant manner. It features a simple syntax and a rich set of built-in functions tailored specifically to observability use cases.
</details>

---
<details>
  <summary> Installation and how to run the file </summary>

</details>

---
<details>
  <summary> Basic structure of the file </summary>

- Path 
  - A path expression is a sequence of period-delimited segments that represent the location of a value within an object. A leading “.” means the path points to the event. A leading “%” means the path points to the event metadata.
  - "."	The . character represents the root of the event. All paths must begin with . or %
  - the dot (.) at the beginning of a path refers to the root of the current event
    
</details>

---
<details>
  <summary> Basic Loops </summary>

- Functions
    - [There are various built-in VRL functions. Functions are categorised by their puropse.](https://vector.dev/docs/reference/vrl/functions/)
    - Main function categories are listed below:
      - Array
      - Codec
      - Coerce
      - Convert
      - Debug
      - Enrichment
      - Enumerate
      - Path
      - Cryptography
      - IP
      - Number
      - Object
      - Parse
      - Random
      - String
      - System
      - Timestamp
      - Type

- Code example to loop through nested data
  - Review the concept first 
    - Path function ==> exists
    - Enumerate function ==> for_each
    - Type function ==> array!
    - Enumerate function ==> unique
    - Array function ==> push

  - Code 


</details>

----
<details>
  <summary>Glossary</summary>
  
- **Vector Remap Language (VRL)**
  - It is an expression-oriented language designed for transforming observability data (logs and metrics) in a safe and performant manner. It features a simple syntax and a rich set of built-in functions tailored specifically to observability use cases.

- **[vector.toml](https://toml.io/en/)**
  - TOML - [Tom's Obvious Minimal Language], A config file format for humans. TOML aims to be a minimal configuration file format that's easy to read due to obvious semantics. TOML is designed to map unambiguously to a hash table. TOML should be easy to parse into data structures in a wide variety of languages.
  - TOML aims to be a minimal configuration file format that: is easy to read due to obvious semantics maps unambiguously to a hash table is easy to parse into data structures in a wide variety of languages
  - TOML has useful native types
    - Key/Value Pairs
    - Arrays
    - Tables
    - Inline tables
    - Arrays of tables
    - Integers & Floats
    - Booleans
    - Dates & Times, with optional offsets
- **vector.toml Example - TB Added**
  - It is a configuration file that defines a dataprocessing pipeline using Vector. It specifies:
    - API configuration
    - A file input source
    - Several transformation steps
    - An output sink to write the processed data to a file

</details>

----
