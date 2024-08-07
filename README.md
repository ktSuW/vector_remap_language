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

- Window
  - Install the latest version of the Vector CLI
  - https://packages.timber.io/vector/0.40.0
    - Download - `vector-0.40.0-x86_64-pc-windows-msvc.zip`
  - unzip it if want to run `vector_local_test.toml` without docker
  - `vector/bin/vector.exe --config vector_local_test.toml`
- Linux
  - 
  
- VRL Playground 
  - https://play.vrl.dev/
  - You can test your code and share 
    - Paste your code into Program
    - Add your log - if it is in JSON format, use JSON tools to minify it and run
    - [Shotern URL since they are very very long if you want to share it with your colleague](https://t.ly/)
    - `https://t.ly/v5zF5`

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

<details>
  <summary> Security related learning </summary>

## Malicious.exe files types

In real-world cyber attacks, there are numerous potentially malicious .exe files that attackers might use. Here are some examples of suspicious or malicious .exe files that security analysts should be aware of:

- Remote Access Tools (RATs):

PoisonIvy.exe
DarkComet.exe
njRAT.exe
QuasarRAT.exe

- Keyloggers:
KeyLogger.exe
PerfectKeylogger.exe
Elite Keylogger.exe

- Trojans:
Zeus.exe
Emotet.exe
TrickBot.exe
Dridex.exe

- Ransomware:
WannaCry.exe
Ryuk.exe
Locky.exe
CryptoLocker.exe

- Cryptocurrency miners:
XMRig.exe
NsCpuCNMiner64.exe

- Backdoors:
Backdoor.exe
Netcat.exe (when used maliciously)

- Exploit kits:
BlackHole.exe
Angler.exe

- Password crackers (when used maliciously):
Mimikatz.exe
HashCat.exe

- Malware droppers:
Downloader.exe
Dropper.exe

- Command and Control (C2) tools:
Cobalt Strike's beacon.exe
Metasploit's meterpreter.exe

- Data exfiltration tools:
Exfil.exe
DataStealer.exe

- Suspicious system tools (when used in unusual contexts):
PsExec.exe
Regsvr32.exe
Rundll32.exe

- Maliciously named system files:
Svchost.exe (in non-system folders)
Explorer.exe (in non-system folders)

- Obfuscated or randomly named executables:
a.exe
1234.exe
temp_[random string].exe

Remember, the presence of these file names doesn't always indicate malicious activity. Legitimate software may use similar names, and malware often uses disguised names to avoid detection. Context is crucial in determining whether an executable is malicious. Factors to consider include:

The file's location
Its digital signature
The process that launched it
Its network activity
Its behavior on the system
Always use multiple indicators and thorough analysis to determine if an executable is truly malicious.

</details>