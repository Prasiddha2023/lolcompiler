<program>::= HAI <program-start> KBYE

<program-start>::= <variable-define> <program-rest> | <program-rest>

<program-rest>::= <comment> <program-rest> | <head> <body>| <body-item> <body>| <empty>

<head>::= MAEK_HEAD <head-start> MKAY

<head-start>::= <variable-define> <head-rest> | <head-rest>

<head-rest>::= <comment> <head-rest> | <title> <comment-list>| <empty>

<title>::= GIMMEH_TITLE <title-start> OIC

<title-start>::= <variable-define> <plain-body> | <plain-body>

<body>::= <comment> <body> | <body-item> <body> | <empty>

<body-item>::= <paragraph> | <list>| <bold>| <link> | <variable-use> | NEWLINE | TEXT

<paragraph>::= MAEK_PARAGRAF <paragraph-start> MKAY

<paragraph-start>::= <variable-define> <paragraph-body> | <paragraph-body>

<paragraph-body>::= <comment> <paragraph-body>| <paragraph-item> <paragraph-body> | <empty>

<paragraph-item>::= <list> | <bold> | <link> | <variable-use> | NEWLINE| TEXT

<bold>::= GIMMEH_BOLD <bold-start> OIC

<bold-start>::= <variable-define> <plain-body> | <plain-body>

<plain-body>::= TEXT <plain-body> | <variable-use> <plain-body>| <empty>

<list>::= MAEK_LIST <list-start> MKAY

<list-start>::= <variable-define> <list-rest> | <list-rest>

<list-rest>::= <comment> <list-rest> | <item> <item-list>

<item-list>::= <item> <item-list> | <empty>

<item>::= GIMMEH_ITEM <item-start> OIC <comment-list>

<item-start>::= <variable-define> <item-body> | <item-body>

<item-body>::= TEXT <item-body> | <variable-use> <item-body> | <bold> <item-body> | <empty>

<link>::= GIMMEH_LINX TEXT OIC

<variable-use>::= GIMMEH_VAR TEXT OIC

<variable-define>::= IHAZ TEXT ITIZ <text-nonempty> MKAY

<comment-list>::= <comment> <comment-list> | <empty>

<comment>::= OBTW <text-list> TLDR

<text-list>::= TEXT <text-list> | <empty>

<text-nonempty>::= TEXT <text-list>

<empty>::=