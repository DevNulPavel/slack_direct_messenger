
// TODO: ???
// Описание модулей используются только в main.rs, lib.rs, mod.rs
// В других файлах надо использовать:
//  - use self:: - для получения чего-то из текущего модуля (например, подмодуля у того же файла), аналог "./"
//  - use super:: - для получения чего-то из родительского модуля, включая приватные элементы, аналог "../"
//  - use crate:: - импорт чего-то от корня крейта, аналог "/"
//
// Модули – не то же самое, что файлы, но существует естественная аналогия между модулями и файлами и каталогами в
// файловой системе Unix. Ключевое слово use создает псевдонимы точно так же, как команда ln создает ссылки.
// Пути, как и имена файлов, могут быть абсолютными и относительными.
// Ключевые слова self и super – аналоги специальных каталогов . и ..
// А extern crate включает в проект корневой модуль еще одного крейта – почти то же, что монтирование файловой системы.
//
// Папку с корневым файлом mod.rs надо воспринимать как модуль с подмодулями в виде других файликов
// другие файлы .rs лучше воспринимать как папку с корневым файлом mod.rs
// https://users.rust-lang.org/t/importing-module-from-another-module/18172/9

pub mod errors;
pub mod user_search;
pub mod messaging;
