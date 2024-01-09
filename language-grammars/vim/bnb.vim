" Vim syntax file
" Language: Bread'n'Butter DSL
" Maintainer: Jean Mertz

if exists("b:current_syntax")
    finish
endif

" Keyword Patterns
syn keyword bnbPlace     place     nextgroup=bnbPlaceName     skipwhite
syn keyword bnbInclude   include   nextgroup=bnbComponentRef  skipwhite
syn keyword bnbComponent component nextgroup=bnbComponentName skipwhite
syn keyword bnbSketch    sketch    nextgroup=bnbSketchPath    skipwhite

" Match Patterns
syn match bnbPlaceName       ".\+" contained
syn match bnbComponentName   ".\+" contained
syn match bnbComponentRef    ".\+" contained
syn match bnbSketchPath      ".\+" contained nextgroup=bnbClickable skipwhite skipempty
syn match bnbAffordance      "\s*\zs[^->]\+" nextgroup=bnbConnection skipwhite
syn match bnbConnection      "->" contained nextgroup=bnbAffordance,bnbConnectionLabel skipwhite
syn match bnbConnectionLabel "(.\{-})" contained
syn match bnbComment         "#.*"
syn region bnbClickable      start="\[" end="\]" nextgroup=bnbAffordance skipwhite

" Define Highlight Groups
hi def link bnbComment         Comment
hi def link bnbPlace           Keyword
hi def link bnbPlaceName       Identifier
hi def link bnbInclude         Include
hi def link bnbComponent       Type
hi def link bnbComponentRef    Type
hi def link bnbComponentName   Identifier
hi def link bnbSketch          PreProc
hi def link bnbSketchPath      Label
hi def link bnbAffordance      String
hi def link bnbConnection      Special
hi def link bnbConnectionLabel Special
hi def link bnbClickable       Constant

let b:current_syntax = "bnb"
