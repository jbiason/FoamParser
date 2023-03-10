OpenFOAM v10 User Guide - 4.2 Basic input/output file format
===========================================================

4.2 Basic input/output file format
---------------------------------

OpenFOAM needs to read a range of data structures such as strings, scalars,
vectors, tensors, lists and fields. The input/output (I/O) format of files is
designed to be extremely ﬂexible to enable the user to modify the I/O in
OpenFOAM applications as easily as possible. The I/O follows a simple set of
rules that make the files extremely easy to understand, in contrast to many
software packages whose file format may not only be difficult to understand
intuitively but also not be published. The OpenFOAM file format is described in
the following sections.

### 4.2.1 General syntax rules

The format follows some general principles of C++ source code.

-   Files have free form, with no particular meaning assigned to any column and
    no need to indicate continuation across lines.
-   Lines have no particular meaning except to a // comment delimiter which
    makes OpenFOAM ignore any text that follows it until the end of line.
-   A comment over multiple lines is done by enclosing the text between `/*` and
    `*/` delimiters.

### 4.2.2 Dictionaries

OpenFOAM uses dictionaries as the most common means of specifying data. A
dictionary is an entity that contains data entries that can be retrieved by the
I/O by means of keywords. The keyword entries follow the general format

```
    <keyword>  <dataEntry1> ... <dataEntryN>;
```

Most entries are single data entries of the form:

```
    <keyword>  <dataEntry>;
```

Most OpenFOAM data files are themselves dictionaries containing a set of keyword
entries. Dictionaries provide the means for organising entries into logical
categories and can be specified hierarchically so that any dictionary can itself
contain one or more dictionary entries. The format for a dictionary is to
specify the dictionary name followed by keyword entries enclosed in curly
braces {} as follows.

```
    <dictionaryName>
    {
        ... keyword entries ...
    }
```

### 4.2.3 The data file header

All data files that are read and written by OpenFOAM begin with a dictionary
named FoamFile containing a standard set of keyword entries, listed below:

-   `version`: I/O format version, optional, defaults to `2.0`
-   `format`: data format, `ascii` or `binary`
-   `class`: class relating to the data, either `dictionary` or a field, e.g. volVectorField
-   `object`: filename, e.g. `controlDict` (mandatory, but not used)
-   `location`: path to the file (optional)

The following example shows the use of keywords to provide data for a case
using the types of entry described so far. The extract, from an fvSolution
dictionary file, contains 2 dictionaries, solvers and PISO. The solvers
dictionary contains multiple data entries for solver and tolerances for each of
the pressure and velocity equations, represented by the p and U keywords
respectively; the PISO dictionary contains algorithm controls.

```
16  
17  solvers
18  {
19      p
20      {
21          solver          PCG;
22          preconditioner  DIC;
23          tolerance       1e-06;
24          relTol          0.05;
25      }
26  
27      pFinal
28      {
29          $p;
30          relTol          0;
31      }
32  
33      U
34      {
35          solver          smoothSolver;
36          smoother        symGaussSeidel;
37          tolerance       1e-05;
38          relTol          0;
39      }
40  }
41  
42  PISO
43  {
44      nCorrectors     2;
45      nNonOrthogonalCorrectors 0;
46      pRefCell        0;
47      pRefValue       0;
48  }
49  
50  
51  // ************************************************************************* //
```

### 4.2.4 Lists

OpenFOAM applications contain lists, e.g. a list of vertex coordinates for a
mesh description. Lists are commonly found in I/O and have a format of their
own in which the entries are contained within round braces `( )`. There is also
a choice of format preceeding the round braces:

-   the keyword is followed immediately by round braces

```
        <listName>     (
            ... entries ...
        );
```

-   the keyword is followed by the number of elements `<n>` in the list

```
        <listName>     <n>
        (
            ... entries ...
        );
```

-   the keyword is followed by a class name identifier `Label<Type>` where
    `<Type>` states what the list contains, e.g. for a list of scalar elements
    is

```
        <listName>     List<scalar>
        <n>        // optional
        (
            ... entries ...
        );
```

Note that `<scalar>` in `List<scalar>` is not a generic name but the actual text
that should be entered.

The simple format is a convenient way of writing a list. The other formats
allow the code to read the data faster since the size of the list can be
allocated to memory in advance of reading the data. The simple format is
therefore preferred for short lists, where read time is minimal, and the other
formats are preferred for long lists.

### 4.2.5 Scalars, vectors and tensors

A scalar is a single number represented as such in a data file. A vector is a
VectorSpace of rank 1 and dimension 3, and since the number of elements is
always fixed to 3, the simple List format is used. Therefore a vector is
written:

```
    (1.0 1.1 1.2)
```

In OpenFOAM, a tensor is a VectorSpace of rank 2 and dimension 3 and therefore
the data entries are always fixed to 9 real numbers. Therefore the identity
tensor can be written:

```
    (
        1 0 0
        0 1 0
        0 0 1
    )
```

This example demonstrates the way in which OpenFOAM ignores the line return is
so that the entry can be written over multiple lines. It is treated no
differently to listing the numbers on a single line:

```
    ( 1 0 0 0 1 0 0 0 1 )
```

### 4.2.6 Dimensional units

In continuum mechanics, properties are represented in some chosen units, e.g.
mass in kilograms, volume in cubic metres, pressure in Pascals. Algebraic
operations must be performed on these properties using consistent units of
measurement; in particular, addition, subtraction and equality are only
physically meaningful for properties of the same dimensional units. As a
safeguard against implementing a meaningless operation, OpenFOAM attaches
dimensions to field data and physical properties and performs dimension
checking on any tensor operation.

The I/O format for a dimensionSet is 7 scalars delimited by square brackets, e.g.

```
    [0 2 -1 0 0 0 0]
```

| No. | Property | SI unit | USCS unit |
| 1 | Mass | kilogram (kg) | pound-mass (lbm) |
| 2 | Length | metre (m) | foot (ft) |
| 3 | Time | second (s) | second (s) |
| 4 | Temperature | Kelvin (K) | degree Rankine  |
| 5 | Quantity | mole (mol) | mole (mol) |
| 6 | Current | ampere (A) | ampere (A) |
| 7 | Luminous intensity | candela (cd) | candela (cd) |

where each of the values corresponds to the power of each of the base units of
measurement listed in Table [4.1]. The table gives the base units for the
Système International (SI) and the United States Customary System (USCS) but
OpenFOAM can be used with any system of units. All that is required is that the
input data is correct for the chosen set of units. It is particularly important
to recognise that OpenFOAM requires some dimensioned physical constants, e.g.
the Universal Gas Constant, for certain calculations, e.g. thermophysical
modelling. These dimensioned constants are specified in a DimensionedConstant
sub-dictionary of main controlDict file of the OpenFOAM installation
`($WM_PROJECT_DIR/etc/controlDict)`. By default these constants are set in SI
units. Those wishing to use the USCS or any other system of units should modify
these constants to their chosen set of units accordingly.

### 4.2.7 Dimensioned types

Physical properties are typically specified with their associated dimensions.
These entries formally have the format that the following example of a
dimensionedScalar demonstrates:

```
    nu             nu  [0 2 -1 0 0 0 0]  1;
```

The first nu is the keyword; the second nu is the word name stored in class
word, usually chosen to be the same as the keyword; the next entry is the
dimensionSet and the final entry is the scalar value.

The majority of dimensioned keyword lookups set a default for the word name
which can therefore be omitted from the entry, so the more common syntax is:

```
    nu             [0 2 -1 0 0 0 0]  1;
```

### 4.2.8 Fields

Much of the I/O data in OpenFOAM are tensor fields, e.g. velocity, pressure
data, that are read from and written into the time directories. OpenFOAM writes
field data using keyword entries as described in
Table [4.2].

| Keyword | Description | Example |
| dimensions | Dimensions of field | [1 1 -2 0 0 0 0] |
| internalField | Value of internal field | uniform (1 0 0) |
| boundaryField | Boundary field | see file listing in section [4.2.8] |

Table 4.2: Main keywords used in field dictionaries.

The data begins with an entry for its dimensions. Following that, is the
internalField, described in one of the following ways.

-   Uniform field a single value is assigned to all elements within the field,
    taking the form:

```
            internalField uniform <entry>;     
```

-   Nonuniform field each field element is assigned a unique value from a list,
    taking the following form where the token identifier form of list is
    recommended:

```
            internalField nonuniform <List>;     
```

The boundaryField is a dictionary containing a set of entries whose names
correspond to each of the names of the boundary patches listed in the boundary
file in the polyMesh directory. Each patch entry is itself a dictionary
containing a list of keyword entries. The mandatory entry, type, describes the
patch field condition specified for the field. The remaining entries correspond
to the type of patch field condition selected and can typically include field
data specifying initial conditions on patch faces. A selection of patch field
conditions available in OpenFOAM are listed in section [5.2.1] ,
section [5.2.2] and section [5.2.3] , with a description and the data that must
be specified with it. Example field dictionary entries for velocity U are shown
below:

```
16  dimensions      [0 1 -1 0 0 0 0];
17  
18  internalField   uniform (0 0 0);
19  
20  boundaryField
21  {
22      movingWall
23      {
24          type            fixedValue;
25          value           uniform (1 0 0);
26      }
27  
28      fixedWalls
29      {
30          type            noSlip;
31      }
32  
33      frontAndBack
34      {
35          type            empty;
36      }
37  }
38  
39  // ************************************************************************* //
```

### 4.2.9 Macro expansion

OpenFOAM dictionary files include a macro syntax to allow convenient
configuration of case files. The syntax uses the dollar ($) symbol in front of
a keyword to expand the data associated with the keyword. For example the value
set for keyword a below, 10, is expanded in the following line, so that the
value of b is also 10.

```
    a 10;
    b $a;
```

Variables can be accessed within different levels of sub-dictionaries, or scope.
Scoping is performed using a '/' (slash) syntax, illustrated by the following
example, where b is set to the value of a, specified in a sub-dictionary called
subdict.

```
    subdictA
    {
        a 20;
    }
    b $subdictA/a;
```

There are further syntax rules for macro expansions:

-   to traverse up one level of sub-dictionary, use the '..' (double-dot) prefix, see below;
-   to traverse up two levels use '../..' prefix, etc.;
-   to traverse to the top level dictionary use the '!' (exclamation mark) prefix (most useful), see below;
-   to traverse into a separate file named otherFile, use 'otherFile!', see below;
-   for multiple levels of macro substitution, each specified with the '$' dollar syntax, '{}' brackets are required to protect the expansion, see below.

When accessing parameters from another file, the `$FOAM_CASE` environment
variable is useful to specify the path to the file as described in
Section [4.2.11] and illustrated below.

```
    a 10;
    b a;
    c ${${b}}; // returns 10, since $b returns "a", and $a returns 10

    subdictA
    {
        a 20;
    }

    subdictB
    {
        // double-dot takes scope up 1 level, then into "subdictA" => 20
        b $../subdictA/a;

        subsubdict
        {
            // exclamation mark takes scope to top level => 10
            b $!a;

            // "a" from another file named "otherFile"
            c $otherFile!a;

            // "a" from another file "otherFile" in the case directory
            d $FOAM_CASE/otherFile!a;
        }
    }
```

### 4.2.10 Including files

There is additional file syntax that provides further ﬂexibility for setting up
of OpenFOAM case files, namely directives. Directives are commands that can be
contained within case files that begin with the hash (#) symbol. The first set
of directive commands are those for reading a data file from within another
data file. For example, let us say a user wishes to set an initial value of
pressure once to be used as the internal field and initial value at a boundary.
We could create a file, e.g. named initialConditions, which contains the
following entries:

```
    pressure 1e+05;
```

In order to use this pressure for both the internal and initial boundary
fields, the user would simply include the initialConditions file using the
#include directive, then use macro expansions for the pressure keyword, as
follows.

```
    #include "initialConditions"
    internalField uniform $pressure;
    boundaryField
    {
        patch1
        {
            type fixedValue;
            value $internalField;
        }
    }
```

The file include directives are as follows:

-   `#include "<path>/<fileName>"`: reads the file of name `<fileName>` from an
    absolute or relative directory path `<path>`;
-   ``#includeIfPresent "<path>/<fileName>"``: reads the file if it exists;
-   ``#includeEtc "<path>/<fileName>"``: reads the file of name `<fileName>`
    from the directory path `<path>`, relative to the `$FOAM_ETC` directory;
-   ``#includeFunc <fileName>``: reads the file of name `<fileName>`, searched
    from the case system directory, followed by the `$FOAM_ETC` directory;
-   ``#remove <keywordEntry>``: removes any included keyword entry; can take a
    word or regular expression;

### 4.2.11 Environment variables

OpenFOAM recognises the use of environment variables in input files. For
example, the `$FOAM_RUN` environment variable can be used to identify the run
directory, as described in the introduction to Chapter [2]. This could be used
to include a file, e.g. by

```
    #include "$FOAM_RUN/pitzDaily/0/U"
```

In addition to environment variables like `$FOAM_RUN`, set within the operating
system, OpenFOAM recognises a number of "internal" environment variables,
including the following.

-   `$FOAM_CASE`: the path and directory of the running case.
-   `$FOAM_CASENAME`: the directory name of the running case.
-   `$FOAM_APPLICATION`: the name of the running application.

### 4.2.12 Regular expressions

When running an application, data is initialised by looking up keywords from
dictionaries. The user can either provide an entry with a keyword that directly
matches the one being looked up, or can provide a [POSIX regular expression]
that matches the keyword, specified inside double-quotations ("..."). Regular
expressions have an extensive syntax for various matches of text patterns but
they are typically only used in the following ways in OpenFOAM input files.

-   `"inlet.*"` matches any word beginning inlet..., including inlet itself,
    because '`.`' denotes "any character" and '`*`' denotes "repeated any number of
    times, including 0 times".
-   `"(inlet|output)"` matches inlet and outlet because `()` specified an
    expression grouping and `|` is an OR operator.

### 4.2.13 Keyword ordering

The order in which keywords are listed does not matter, except when the same
keyword is specified multiple times. Where the same keyword is duplicated, the
last instance is used. The most common example of a duplicate keyword occurs
when a keyword is included from the file or expanded from a macro, and then
overridden. The example below demonstrates this, where pFinal adopts all the
keyword entries, including relTol 0.05 in the p sub-dictionary by the macro
expansion $p, then overrides the relTol entry.

```
    p
    {
        solver          PCG;
        preconditioner  DIC;
        tolerance       1e-6;
        relTol          0.05;
    }
    pFinal
    {
        $p;
        relTol          0;
    }
```

Where a data lookup matches both a keyword and a regular expression, the
keyword match takes precedence irrespective of the order of the entries.

### 4.2.14 Inline calculations and code

There are two further directives that enable calculations from within input
files: #calc, for simple calculations; #codeStream, for more complex
calculations.

The pipeCyclic tutorial in `$FOAM_TUTORIALS/incompressible/simpleFoam`
demonstrates the #calc directive through its blockMesh configuration in
blockMeshDict:

```
    //- Half angle of wedge in degrees
    halfAngle 45.0;

    //- Radius of pipe [m]
    radius 0.5;

    radHalfAngle    #calc "degToRad($halfAngle)";
    y               #calc "$radius*sin($radHalfAngle)";
    z               #calc "$radius*cos($radHalfAngle)";
```

The file contains several calculations that calculate vertex ordinates, e.g. y,
z, etc., from geometry dimensions, e.g. radius. Calculations include standard
C++ functions including unit conversions, e.g. degToRad, and trigonometric
functions, e.g. sin.

The #codeStream directive takes C++ code which is compiled and executed to
deliver the dictionary entry. The code and compilation instructions are
specified through the following keywords.

-   code: specifies the code, called with arguments OStream& os and const
    dictionary& dict which the user can use in the code, e.g. to lookup keyword
    entries from within the current case dictionary (file).
-   codeInclude (optional): specifies additional C++ #include statements to
    include OpenFOAM files.
-   codeOptions (optional): specifies any extra compilation ﬂags to be added to
    `EXE_INC` in Make/options.
-   codeLibs (optional): specifies any extra compilation ﬂags to be added to
    `LIB_LIBS` in Make/options.

Code, like any string, can be written across multiple lines by enclosing it
within hash-bracket delimiters, i.e. #{...#}. Anything in between these two
delimiters becomes a string with all newlines, quotes, etc. preserved.

An example of #codeStream is given below, where the code in the calculates
moment of inertia of a box shaped geometry.

```
momentOfInertia #codeStream
{
    codeInclude
    #{
        #include "diagTensor.H"
    #};

    code
    #{
        scalar sqrLx = sqr($Lx);
        scalar sqrLy = sqr($Ly);
        scalar sqrLz = sqr($Lz);
        os  <<
            $mass
           *diagTensor(sqrLy + sqrLz, sqrLx + sqrLz, sqrLx + sqrLy)/12.0;
    #};
};
```

### 4.2.15 Conditionals

Input files support two conditional directives: #if...#else...#endif; and,
#ifEq... #else... #endif. The #if conditional reads a switch that can be
generated by a #calc directive, e.g.:

```
angle 65;

laplacianSchemes
{
#if #calc "${angle} < 75"
    default  Gauss linear corrected;
#else
    default  Gauss linear limited corrected 0.5;
#endif
}
```

The #ifEq compares a word or string, and executes based on a match, e.g.:

```
ddtSchemes
{
#ifeq ${FOAM_APPLICATION} simpleFoam
    default         steadyState;
#else
    default         Euler;
#endif
}
```
