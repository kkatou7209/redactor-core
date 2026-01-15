# Index

- [Index](#index)
- [Directory Explanation](#directory-explanation)
  - [Overview](#overview)
  - [`src/object` Directory](#srcobject-directory)
  - [`src/token` Directory](#srctoken-directory)
  - [`src/util` Directory](#srcutil-directory)
  - [`src/value` Directory](#srcvalue-directory)

# Directory Explanation

## Overview

- `src/object`
  
  This directory contains modules representing various PDF object types, such as Boolean objects.

- `src/token`
  
  This directory contains modules related to PDF tokenization, including delimiters and whitespace handling.

- `src/util`
  
  This directory contains utility functions, including checks for PDF byte validity.

- `src/value`
  
  This directory contains modules for different PDF value types, such as characters.

## `src/object` Directory

This directory contains modules that define various PDF object types. 

For example, the `boolean.rs` file defines a `Boolean` struct that represents a PDF Boolean object, along with its associated methods and traits.

## `src/token` Directory

This directory includes modules that handle the tokenization of PDF files. 

It contains files like `delimiter.rs` and `whitespace.rs`, which manage the parsing of delimiters and whitespace characters in PDF content.

## `src/util` Directory

This directory houses utility functions that assist in various operations related to PDF processing.

For instance, the `check.rs` file provides functions to validate PDF byte characters, such as checking if a byte is a valid name character in a PDF Name object.

## `src/value` Directory

This directory contains modules that define different PDF value types. 

For example, the `char.rs` file likely defines a character type used in PDF values, along with its associated functionality.

<style>

h1:not(:first-child) {
    margin-top: 2em;
}
</style>