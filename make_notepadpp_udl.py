#!/usr/bin/env python3
"""
Script to generate a Notepad++ UDL (User Defined Language) file for the Verbena scripting language.
The script extracts keywords from src/parser.rs and standard library functions from src/stdlib.rs.
"""

import os
import re
import sys


def extract_keywords(parser_file):
    """Extract keywords from the parser.rs file."""
    keywords = []
    if not os.path.exists(parser_file):
        print(
            f"Warning: Parser file {parser_file} not found. Using empty keywords list."
        )
        return keywords

    with open(parser_file, "r") as f:
        content = f.read()

    # Pattern to match keyword insertions
    pattern = r'keywords\.insert\("([^"]+)"\.to_string\(\), Tok::[^)]+\);'
    matches = re.findall(pattern, content)

    return matches


def extract_stdlib_functions(stdlib_file):
    """Extract standard library functions from the stdlib.rs file."""
    functions = []
    if not os.path.exists(stdlib_file):
        print(
            f"Warning: StdLib file {stdlib_file} not found. Using empty functions list."
        )
        return functions

    with open(stdlib_file, "r") as f:
        content = f.read()

    # Pattern to match function registrations
    pattern = r'vm\.register\d+\("([^"]+)", [^)]+\);'
    matches = re.findall(pattern, content)

    return matches


def generate_udl_file(output_file, keywords, functions):
    """Generate the Notepad++ UDL XML file."""
    udl_template = f"""<NotepadPlus>
    <UserLang name="Verbena" ext="va" udlVersion="2.1">
        <Settings>
            <Global caseIgnored="no" allowFoldOfComments="no" foldCompact="no" forcePureLC="0" decimalSeparator="0" />
            <Prefix Keywords1="no" Keywords2="no" Keywords3="no" Keywords4="no" Keywords5="no" Keywords6="no" Keywords7="no" Keywords8="no" />
        </Settings>
        <KeywordLists>
            <Keywords name="Comments">00# 01 02 03 04</Keywords>
            <Keywords name="Numbers, prefix1"></Keywords>
            <Keywords name="Numbers, prefix2"></Keywords>
            <Keywords name="Numbers, extras1"></Keywords>
            <Keywords name="Numbers, extras2"></Keywords>
            <Keywords name="Numbers, suffix1"></Keywords>
            <Keywords name="Numbers, suffix2"></Keywords>
            <Keywords name="Numbers, range"></Keywords>
            <Keywords name="Operators1">+ - * / % ^ &amp; | ~ ! = &lt; &gt; , ; : . ( ) [ ] {{ }}</Keywords>
            <Keywords name="Operators2"></Keywords>
            <Keywords name="Folders in code1, open"></Keywords>
            <Keywords name="Folders in code1, middle"></Keywords>
            <Keywords name="Folders in code1, close"></Keywords>
            <Keywords name="Folders in code2, open"></Keywords>
            <Keywords name="Folders in code2, middle"></Keywords>
            <Keywords name="Folders in code2, close"></Keywords>
            <Keywords name="Folders in comment, open"></Keywords>
            <Keywords name="Folders in comment, middle"></Keywords>
            <Keywords name="Folders in comment, close"></Keywords>
            <Keywords name="Keywords1">{' '.join(keywords)}</Keywords>
            <Keywords name="Keywords2">{' '.join(functions)}</Keywords>
            <Keywords name="Keywords3"></Keywords>
            <Keywords name="Keywords4"></Keywords>
            <Keywords name="Keywords5"></Keywords>
            <Keywords name="Keywords6"></Keywords>
            <Keywords name="Keywords7"></Keywords>
            <Keywords name="Keywords8"></Keywords>
            <Keywords name="Delimiters">00&quot; 01\\ 02&quot; 03' 04\\ 05' 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23</Keywords>
        </KeywordLists>
        <Styles>
            <WordsStyle name="DEFAULT" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="COMMENTS" fgColor="008000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="LINE COMMENTS" fgColor="008000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="NUMBERS" fgColor="FF8000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="KEYWORDS1" fgColor="0000FF" bgColor="FFFFFF" fontName="" fontStyle="1" nesting="0" />
            <WordsStyle name="KEYWORDS2" fgColor="800080" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="KEYWORDS3" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="KEYWORDS4" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="KEYWORDS5" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="KEYWORDS6" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="KEYWORDS7" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="KEYWORDS8" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="OPERATORS" fgColor="000080" bgColor="FFFFFF" fontName="" fontStyle="1" nesting="0" />
            <WordsStyle name="FOLDER IN CODE1" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="FOLDER IN CODE2" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="FOLDER IN COMMENT" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="DELIMITERS1" fgColor="808080" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="DELIMITERS2" fgColor="808080" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="DELIMITERS3" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="DELIMITERS4" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="DELIMITERS5" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="DELIMITERS6" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="DELIMITERS7" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
            <WordsStyle name="DELIMITERS8" fgColor="000000" bgColor="FFFFFF" fontName="" fontStyle="0" nesting="0" />
        </Styles>
    </UserLang>
</NotepadPlus>
"""

    with open(output_file, "w", newline="\n") as f:
        f.write(udl_template)

    print(f"UDL file generated successfully: {output_file}")
    print(f"You can now restart Notepad++ to use the Verbena language highlighting.")


def main():
    parser_file = "src/parser.rs"
    stdlib_file = "src/stdlib.rs"

    # Determine the output path using environment variables
    appdata = os.environ.get("APPDATA")
    if appdata:
        output_dir = os.path.join(appdata, "Notepad++", "userDefineLangs")
        if not os.path.exists(output_dir):
            os.makedirs(output_dir, exist_ok=True)
        output_file = os.path.join(output_dir, "verbena.xml")
    else:
        print(
            "Warning: APPDATA environment variable not found. Using current directory."
        )
        output_file = "verbena.xml"

    # Extract keywords and functions
    keywords = extract_keywords(parser_file)
    functions = extract_stdlib_functions(stdlib_file)

    # Print summary
    print(
        f"Found {len(keywords)} keywords and {len(functions)} standard library functions."
    )

    # Generate UDL file
    generate_udl_file(output_file, keywords, functions)


if __name__ == "__main__":
    main()
