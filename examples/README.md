This file contains an overview of examples.

- `derive` folder contains a list of examples which uses different `#[derive(Tabled)]` attributes.
- `show` folder contains a program which uses different styles and prints the resulting table.
- `table_terminal_width` folder contains a program which spreads the table to the max terminal width.

Bellow there's a list of results of running some examples.

## basic

```
| name    | based_on | is_active | is_cool |
|---------|----------|-----------|---------|
| Manjaro | Arch     | true      | true    |
| Arch    |          | true      | true    |
| Debian  |          | true      | true    |
```

## border_text

```
 Numbers ─┬────┬────┬────┐
│ 0  │ 1  │ 2  │ 3  │ 4  │
 More numbers ─┼────┼────┤
│ 5  │ 6  │ 7  │ 8  │ 9  │
│ 10 │ 11 │ 12 │ 13 │ 14 │
 end. ────┴────┴────┴────┘
```

## builder_index

```
┌───────────┬─────────┬──────┬────────┐
│           │ Manjaro │ Arch │ Debian │
├───────────┼─────────┼──────┼────────┤
│ based_on  │ Arch    │ None │ None   │
├───────────┼─────────┼──────┼────────┤
│ is_active │ true    │ true │ true   │
├───────────┼─────────┼──────┼────────┤
│ is_cool   │ true    │ true │ true   │
└───────────┴─────────┴──────┴────────┘
```

## builder

```
| https://en.wikipedia.org/wiki/Ocean |
|---------------+---------------------|
| The terms "the ocean" or "the sea"  |
| used without specification refer to |
|  the interconnected body of salt wa |
| ter covering the majority of the Ea |
| rth's surface                       |
| =================================== |
| #             | Ocean               |
| 0             | Atlantic            |
| 1             | Pacific             |
| 2             | Indian              |
| 3             | Southern            |
| 4             | Arctic              |
```

## col_row_macros

```
+-------------------------------------------+---------------------------------------------+
| .---------------------------------------. | ┌────────────────────┬─────┬──────────────┐ |
| | name             | age | is_validated | | │ name               │ age │ is_validated │ |
| | Jon Doe          | 255 | false        | | ├────────────────────┼─────┼──────────────┤ |
| | Mark Nelson      | 13  | true         | | │ Jack Black         │ 51  │ false        │ |
| | Terminal Monitor | 0   | false        | | ├────────────────────┼─────┼──────────────┤ |
| | Adam Blend       | 17  | true         | | │ Michelle Goldstein │ 44  │ true         │ |
| '---------------------------------------' | └────────────────────┴─────┴──────────────┘ |
+-------------------------------------------+---------------------------------------------+
+-------------------------------------------+
| .---------------------------------------. |
| | name             | age | is_validated | |
| | Jon Doe          | 255 | false        | |
| | Mark Nelson      | 13  | true         | |
| | Terminal Monitor | 0   | false        | |
| | Adam Blend       | 17  | true         | |
| '---------------------------------------' |
+-------------------------------------------+
| .---------------------------------------. |
| | name             | age | is_validated | |
| | Jon Doe          | 255 | false        | |
| | Mark Nelson      | 13  | true         | |
| | Terminal Monitor | 0   | false        | |
| | Adam Blend       | 17  | true         | |
| '---------------------------------------' |
+-------------------------------------------+
| .---------------------------------------. |
| | name             | age | is_validated | |
| | Jon Doe          | 255 | false        | |
| | Mark Nelson      | 13  | true         | |
| | Terminal Monitor | 0   | false        | |
| | Adam Blend       | 17  | true         | |
| '---------------------------------------' |
+-------------------------------------------+
+-------------------------------------------------------------------------------+
|  +-------+-----+--------------+  ┌────────────────────┬─────┬──────────────┐  |
|  | name  | age | is_validated |  │ name               │ age │ is_validated │  |
|  +-------+-----+--------------+  ├────────────────────┼─────┼──────────────┤  |
|  | Sam   | 31  | true         |  │ Jack Black         │ 51  │ false        │  |
|  +-------+-----+--------------+  ├────────────────────┼─────┼──────────────┤  |
|  | Sarah | 26  | true         |  │ Michelle Goldstein │ 44  │ true         │  |
|  +-------+-----+--------------+  └────────────────────┴─────┴──────────────┘  |
+-------------------------------------------------------------------------------+
|                   .---------------------------------------.                   |
|                   | name             | age | is_validated |                   |
|                   | Jon Doe          | 255 | false        |                   |
|                   | Mark Nelson      | 13  | true         |                   |
|                   | Terminal Monitor | 0   | false        |                   |
|                   | Adam Blend       | 17  | true         |                   |
|                   '---------------------------------------'                   |
+-------------------------------------------------------------------------------+
```

## color

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/zhiburt/tabled/assets/assets/example-color-1-dark.png">
  <img alt="Preview" src="https://raw.githubusercontent.com/zhiburt/tabled/assets/assets/example-color-1-light.png">
</picture>

## colored_borders

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/zhiburt/tabled/assets/assets/example-colored_borders-1-dark.png">
  <img alt="Preview" src="https://raw.githubusercontent.com/zhiburt/tabled/assets/assets/example-colored_borders-1-light.png">
</picture>

## concat

```
 temperature_c  wind_ms  latitude  longitude 
 16             3000     111.111   333.333   
 -20            300      5.111     7282.1    
 40             100      0         0         
                         0         0        
```

## custom_style

```
┌─────────────────────────────────────────────────────┐
│ name                first_release  developer        │
├─────────────────────────────────────────────────────┤
│ Sublime Text 3      2008           Sublime HQ       │
│ Visual Studio Code  2015           Microsoft        │
│ Notepad++           2003           Don Ho           │
│ GNU Emacs           1984           Richard Stallman │
│ Neovim              2015           Vim community    │
└─────────────────────────────────────────────────────┘
```

## expanded_display

```
-[ RECORD 0 ]------
name      | Manjaro
based_on  | Arch
is_active | true
is_cool   | true
-[ RECORD 1 ]------
name      | Arch
based_on  | 
is_active | true
is_cool   | true
-[ RECORD 2 ]------
name      | Debian
based_on  | 
is_active | true
is_cool   | true
```

## extract

```
┌───────────────┬───────────────────────────┬──────────────────┬────────────────────┐
│    artist     │           name            │     released     │ level_of_greatness │
├───────────────┼───────────────────────────┼──────────────────┼────────────────────┤
│ Pink Floyd    │ The Dark Side of the Moon │ 01 March 1973    │ Unparalleled       │
├───────────────┼───────────────────────────┼──────────────────┼────────────────────┤
│ Fleetwood Mac │ Rumours                   │ 04 February 1977 │ Outstanding        │
├───────────────┼───────────────────────────┼──────────────────┼────────────────────┤
│ Led Zeppelin  │ Led Zeppelin IV           │ 08 November 1971 │ Supreme            │
└───────────────┴───────────────────────────┴──────────────────┴────────────────────┘

┼───────────────────────────┼──────────────────┼──────────────┤
│ The Dark Side of the Moon │ 01 March 1973    │ Unparalleled │
┼───────────────────────────┼──────────────────┼──────────────┤
│ Rumours                   │ 04 February 1977 │ Outstanding  │
┼───────────────────────────┼──────────────────┼──────────────┤

┌───────────────────────────┬──────────────────┬───────────────┐
│ The Dark Side of the Moon │ 01 March 1973    │ Unparalleled  │
├───────────────────────────┼──────────────────┼───────────────┤
│ Rumours                   │ 04 February 1977 │ +Outstanding+ │
└───────────────────────────┴──────────────────┴───────────────┘
```

## format

```
 0                                           | 1                              | 2                       
---------------------------------------------+--------------------------------+-------------------------
 8ae4e8957caeaa467acbce963701e227af00a1c7... | bypass open-source transmitter | index neural panel      
 48c76de71bd685486d97dc8f4f05aa6fcc0c3f86... | program online alarm           | copy bluetooth card     
 6ffc2a2796229fc7bf59471ad907f58b897005d0... | CSV                            | reboot mobile capacitor 
```

## formatting_settings

```
╭───────────────────╮
│       &str        │
├───────────────────┤
│                   │
│ [                 │
│     "foo",        │
│     {             │
│         "bar": 1, │
│         "baz": [  │
│             2,    │
│             3     │
│         ]         │
│     }             │
│ ]                 │
╰───────────────────╯

╭───────────────────╮
│       &str        │
├───────────────────┤
│                   │
│         [         │
│        "foo",     │
│           {       │
│         "bar": 1, │
│         "baz": [  │
│              2,   │
│               3   │
│             ]     │
│           }       │
│         ]         │
╰───────────────────╯

╭───────────────────╮
│       &str        │
├───────────────────┤
│     [             │
│     "foo",        │
│     {             │
│     "bar": 1,     │
│     "baz": [      │
│     2,            │
│     3             │
│     ]             │
│     }             │
│     ]             │
│                   │
╰───────────────────╯
```

## highlight

```
*************
* 0 │ 1 │ 2 *
*****───*****
│ A * B * C │
├───*───*───┤
│ D * E * F │
├───*───*───┤
│ G * H * I │
└───*****───┘
```

## margin

```
vvvvvvvvvvvvvvvvvv
vvvvvvvvvvvvvvvvvv
<<<<=== === ===>>>
<<<< 0   1   2 >>>
<<<<=== === ===>>>
<<<< A   B   C >>>
<<<< D   E   F >>>
<<<< G   H   I >>>
<<<<=== === ===>>>
^^^^^^^^^^^^^^^^^^
```

## nested_table

```
+-----------------------------------------------+
|            +---------------------+            |
|            |       Animal        |            |
|            +---------------------+            |
|            | +-----------------+ |            |
|            | | +age: Int       | |            |
|            | | +gender: String | |            |
|            | +-----------------+ |            |
|            | +-----------------+ |            |
|            | | +isMammal()     | |            |
|            | | +mate()         | |            |
|            | +-----------------+ |            |
|            +---------------------+            |
|                       ▲                       |
|                       |                       |
|                       |                       |
|     +-----------------------------------+     |
|     |               Duck                |     |
|     +-----------------------------------+     |
|     | +-------------------------------+ |     |
|     | | +beakColor: String = "yellow" | |     |
|     | +-------------------------------+ |     |
|     | +-------------------------------+ |     |
|     | | +swim()                       | |     |
|     | | +quack()                      | |     |
|     | +-------------------------------+ |     |
|     +-----------------------------------+     |
+-----------------------------------------------+
```

## nested_table_2

```
┌───────┬─────────────────────────────────────────────────┬──────────────────────────────────────────────┐
│ name  │ main_os                                         │ switch_os                                    │
├───────┼─────────────────────────────────────────────────┼──────────────────────────────────────────────┤
│ Azure │ ╔═════════╦═════════════╦═══════════╦═════════╗ │ ╔═════════╦══════════╦═══════════╦═════════╗ │
│       │ ║ name    ║ based_on    ║ is_active ║ is_cool ║ │ ║ name    ║ based_on ║ is_active ║ is_cool ║ │
│       │ ╠═════════╬═════════════╬═══════════╬═════════╣ │ ╠═════════╬══════════╬═══════════╬═════════╣ │
│       │ ║ Windows ║ Independent ║ true      ║ true    ║ │ ║ Manjaro ║ Arch     ║ true      ║ true    ║ │
│       │ ╚═════════╩═════════════╩═══════════╩═════════╝ │ ╚═════════╩══════════╩═══════════╩═════════╝ │
├───────┼─────────────────────────────────────────────────┼──────────────────────────────────────────────┤
│ AWS   │ ╔════════╦═════════════╦═══════════╦═════════╗  │ ╔══════╦═════════════╦═══════════╦═════════╗ │
│       │ ║ name   ║ based_on    ║ is_active ║ is_cool ║  │ ║ name ║ based_on    ║ is_active ║ is_cool ║ │
│       │ ╠════════╬═════════════╬═══════════╬═════════╣  │ ╠══════╬═════════════╬═══════════╬═════════╣ │
│       │ ║ Debian ║ Independent ║ true      ║ true    ║  │ ║ Arch ║ Independent ║ true      ║ true    ║ │
│       │ ╚════════╩═════════════╩═══════════╩═════════╝  │ ╚══════╩═════════════╩═══════════╩═════════╝ │
├───────┼─────────────────────────────────────────────────┼──────────────────────────────────────────────┤
│ GCP   │ ╔════════╦═════════════╦═══════════╦═════════╗  │ ╔══════╦═════════════╦═══════════╦═════════╗ │
│       │ ║ name   ║ based_on    ║ is_active ║ is_cool ║  │ ║ name ║ based_on    ║ is_active ║ is_cool ║ │
│       │ ╠════════╬═════════════╬═══════════╬═════════╣  │ ╠══════╬═════════════╬═══════════╬═════════╣ │
│       │ ║ Debian ║ Independent ║ true      ║ true    ║  │ ║ Arch ║ Independent ║ true      ║ true    ║ │
│       │ ╚════════╩═════════════╩═══════════╩═════════╝  │ ╚══════╩═════════════╩═══════════╩═════════╝ │
└───────┴─────────────────────────────────────────────────┴──────────────────────────────────────────────┘
```

## nested_table_3

```
*************************************************
*                   Thank You                   *
*************************************************
| +------------+------------------------------+ |
| |               Contributors                | |
| +------------+------------------------------+ |
| |   author   |           profile            | |
| +------------+------------------------------+ |
| |   kozmod   |   https:/github.com/kozmod   | |
| +------------+------------------------------+ |
| | IsaacCloos | https:/github.com/IsaacCloos | |
| +------------+------------------------------+ |
|  +-----------+-----------------------------+  |
|  |                 Issuers                 |  |
|  +-----------+-----------------------------+  |
|  |  author   |           profile           |  |
|  +-----------+-----------------------------+  |
|  | aharpervc | https:/github.com/aharpervc |  |
|  +-----------+-----------------------------+  |
+-----------------------------------------------+
```

## padding_color

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/zhiburt/tabled/assets/assets/example-padding_color-1-dark.png">
  <img alt="Preview" src="https://raw.githubusercontent.com/zhiburt/tabled/assets/assets/example-padding_color-1-light.png">
</picture>

## panel

```
┌───┬────────────────────────────────────────────────────────────────────┬───┐
│ S │                          Tabled Releases                           │ S │
│ o │                                                                    │ o │
│ m │                                                                    │ m │
│ e │                                                                    │ e │
│   ├─────────┬────────────────┬───────────┬─────────────────────────────┤   │
│ t │ version │ published_date │ is_active │        major_feature        │ t │
│ e │         │                │           │                             │ e │
│ x ├─────────┼────────────────┼───────────┼─────────────────────────────┤ x │
│ t │  0.2.1  │   2021-06-23   │   true    │ #[header(inline)] attribute │ t │
│   │         │                │           │                             │   │
│ g ├─────────┼────────────────┼───────────┼─────────────────────────────┤ g │
│ o │  0.2.0  │   2021-06-19   │   false   │         API changes         │ o │
│ e │         │                │           │                             │ e │
│ s ├─────────┼────────────────┼───────────┼─────────────────────────────┤ s │
│   │  0.1.4  │   2021-06-07   │   false   │   display_with attribute    │   │
│ h │         │                │           │                             │ h │
│ e ├─────────┴────────────────┴───────────┴─────────────────────────────┤ e │
│ r │                               N - 3                                │ r │
│ e │                                                                    │ e │
└───┴────────────────────────────────────────────────────────────────────┴───┘
```

## print_matrix

```
┌────┬────┬────┬────┬────┬────┬────┬────┬────┬─────┐
│ 0  │ 1  │ 2  │ 3  │ 4  │ 5  │ 6  │ 7  │ 8  │  9  │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼─────┤
│ 1  │ 2  │ 3  │ 4  │ 5  │ 6  │ 7  │ 8  │ 9  │ 10  │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼─────┤
│ 2  │ 4  │ 6  │ 8  │ 10 │ 12 │ 14 │ 16 │ 18 │ 20  │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼─────┤
│ 3  │ 6  │ 9  │ 12 │ 15 │ 18 │ 21 │ 24 │ 27 │ 30  │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼─────┤
│ 4  │ 8  │ 12 │ 16 │ 20 │ 24 │ 28 │ 32 │ 36 │ 40  │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼─────┤
│ 5  │ 10 │ 15 │ 20 │ 25 │ 30 │ 35 │ 40 │ 45 │ 50  │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼─────┤
│ 6  │ 12 │ 18 │ 24 │ 30 │ 36 │ 42 │ 48 │ 54 │ 60  │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼─────┤
│ 7  │ 14 │ 21 │ 28 │ 35 │ 42 │ 49 │ 56 │ 63 │ 70  │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼─────┤
│ 8  │ 16 │ 24 │ 32 │ 40 │ 48 │ 56 │ 64 │ 72 │ 80  │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼─────┤
│ 9  │ 18 │ 27 │ 36 │ 45 │ 54 │ 63 │ 72 │ 81 │ 90  │
├────┼────┼────┼────┼────┼────┼────┼────┼────┼─────┤
│ 10 │ 20 │ 30 │ 40 │ 50 │ 60 │ 70 │ 80 │ 90 │ 100 │
└────┴────┴────┴────┴────┴────┴────┴────┴────┴─────┘
```

## rotate

```
+--------------+------------------------+---------------------------+--------------------------+
| link         | https://getfedora.org/ | https://www.opensuse.org/ | https://endeavouros.com/ |
+--------------+------------------------+---------------------------+--------------------------+
| destribution | Fedora                 | OpenSUSE                  | Endeavouros              |
+--------------+------------------------+---------------------------+--------------------------+
| id           | 0                      | 2                         | 3                        |
+--------------+------------------------+---------------------------+--------------------------+
```

## span

```
┌───────────────────────────────────────────────────────────────────────────────┐
│ span all 5 columns                                                            │
├───────────────────────────────────────────────────────────────┬───────────────┤
│ span 4 columns                                                │ just 1 column │
├───────────────────────────────┬───────────────┬───────────────┼───────────────┤
│ span 2 columns                │ just 1 column │               │               │
├───────────────┬───────────────┴───────────────┤ just 1 column │               │
│ just 1 column │ span 2 columns                │ span          │ just 1 column │
│               │ span                          │ 3             │ span          │
├───────────────┤ 2                             │ columns       │ 4             │
│ just 1 column │ columns                       │               │ columns       │
├───────────────┼───────────────┬───────────────┼───────────────┤               │
│ just 1 column │ just 1 column │ just 1 column │ just 1 column │               │
└───────────────┴───────────────┴───────────────┴───────────────┴───────────────┘
```

## table_width

```
| 0                | 1         |
|------------------|-----------|
| Hello World!!!   | 3.3.22.2  |
| Guten Morgen     | 1.1.1.1   |
| Добры вечар      | 127.0.0.1 |
| Bonjour le monde |           |
| Ciao mondo       |           |

| 0          | 1   |
|------------|-----|
| Hello W... | ... |
| Guten M... | ... |
| Добры в... | ... |
| Bonjour... |     |
| Ciao mondo |     |

| 0     | 1   |
|-------|-----|
| Hello | ... |
|  W... |     |
| Guten | ... |
|  M... |     |
| Добры | ... |
|  в... |     |
| Bonjo |     |
| ur... |     |
| Ciao  |     |
| mondo |     |

| 0             | 1          |
|---------------|------------|
| Hello         | ...        |
|  W...         |            |
| Guten         | ...        |
|  M...         |            |
| Добры         | ...        |
|  в...         |            |
| Bonjo         |            |
| ur...         |            |
| Ciao          |            |
| mondo         |            |
```

## table_width_2

```
.----------------------------------------.
| usize | &str                           |
| 0     | # Changelog                    |
| 1     | All notable changes to this    |
|       | projectwill be documented in   |
|       | thisfile.                      |
| 2     | The format is based on [Keep a |
|       | Changelog](https://keepachange |
|       | log.com/en/1.0.0/),            |
| 3     | and this project adheres to    |
|       | [SemanticVersioning](https://s |
|       | emver.org/spec/v2.0.0.html).   |
| 4     | ## Unreleased                  |
'-------+--------------------------------'
```


