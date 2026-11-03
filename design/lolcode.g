grammar lolcode;

program        : HAI programStart KBYE EOF;
programStart   : variableDefine programRest | programRest;
programRest    : comment programRest | head body | bodyItem body | ;

head           : MAEK_HEAD headStart MKAY;
headStart      : variableDefine headRest | headRest;
headRest       : comment headRest | title commentList | ;

title          : GIMMEH_TITLE titleStart OIC;
titleStart     : variableDefine plainBody | plainBody;

body           : comment body | bodyItem body | ;
bodyItem       : paragraph | list | bold | link | varUse | NEWLINE | TEXT;

paragraph      : MAEK_PARAGRAF paragraphStart MKAY;
paragraphStart : variableDefine paragraphBody | paragraphBody;
paragraphBody  : comment paragraphBody | paragraphItem paragraphBody | ;
paragraphItem  : list | bold | link | varUse | NEWLINE | TEXT;

bold           : GIMMEH_BOLD boldStart OIC;
boldStart      : variableDefine plainBody | plainBody;
plainBody      : TEXT plainBody | varUse plainBody | ;

list           : MAEK_LIST listStart MKAY;
listStart      : variableDefine listRest | listRest;
listRest       : comment listRest | item itemList;
itemList       : item itemList | ;

item           : GIMMEH_ITEM itemStart OIC commentList;
itemStart      : variableDefine itemBody | itemBody;
itemBody       : TEXT itemBody | varUse itemBody | bold itemBody | ;

link           : GIMMEH_LINX TEXT OIC;
varUse         : GIMMEH_VAR TEXT OIC;
variableDefine : IHAZ TEXT ITIZ textNonempty MKAY;

commentList    : comment commentList | ;
comment        : OBTW textList TLDR;
textList       : TEXT textList | ;
textNonempty   : TEXT textList;

HAI            : '#HAI' | '#hai';
KBYE           : '#KBYE' | '#kbye';
OBTW           : '#OBTW' | '#obtw';
TLDR           : '#TLDR' | '#tldr';
MKAY           : '#MKAY' | '#mkay';
OIC            : '#OIC' | '#oic';
NEWLINE        : '#NEWLINE' | '#newline';
IHAZ           : '#IHAZ' | '#ihaz';
ITIZ           : '#ITIZ' | '#itiz';
MAEK_HEAD      : '#MAEK HEAD' | '#maek head';
MAEK_PARAGRAF  : '#MAEK PARAGRAF' | '#maek paragraf';
MAEK_LIST      : '#MAEK LIST' | '#maek list';
GIMMEH_TITLE   : '#GIMMEH TITLE' | '#gimmeh title';
GIMMEH_BOLD    : '#GIMMEH BOLD' | '#gimmeh bold';
GIMMEH_ITEM    : '#GIMMEH ITEM' | '#gimmeh item';
GIMMEH_LINX    : '#GIMMEH LINX' | '#gimmeh linx';
GIMMEH_VAR     : '#GIMMEH VAR' | '#gimmeh var';

TEXT           : ('A'..'Z' | 'a'..'z' | '0'..'9' | ',' | '.' | '"' | ':' | '?' | '!' | '%' | '=' | '/')+;
WS             : (' ' | '\t' | '\r' | '\n')+ { $channel = HIDDEN; };
