# Symulator Spadającego Piasku

## Opis
Jest to interaktywna symulacja fizyki cząsteczek(piasek(element sypki) woda(ciecz) i drewno(ciało stałe)) w języku Rust przy użyciu silnika gier GGEZ, Każdy materiał zachowuje się zgodnie z uproszczonymi zasadami fizyki, tworząć ciekawe i dynamiczne scenariusze.

## Funkcje 
* Trzy różne materiały do wyboru:
 * Piasek: Opada w dół i układa się w stożki
 * Woda: Płynie i rozprzesrzenia się poziomo
 * Drewno: Tworzy stałe struktury
* Interaktywny interfejs użytkownika z przyciskami do wyboru materiału
* Płynne dodawanie cząsteczek poprzez kliknięcie i przeciągniecie myszą

## Wymagania 
* Rust
* Cargo
* Bibliotekiu:
	* ggez
	* rand

## Parametry konfiguracyjne

W kodzie można dostosować następujące stałe:
```rust
const GRID_WIDTH: usize = 50;    // Szerokość siatki symulacji
const GRID_HEIGHT: usize = 50;   // Wysokość siatki symulacji
const CELL_SIZE: f32 = 5.0;      // Rozmiar pojedynczej komórki w pikselach
const UI_HEIGHT: f32 = 40.0;     // Wysokość paska interfejsu użytkownika
```

## Znane problemy
* Przy bardzo dużej liczbie cząsteczek może wystąpić spowolnienie symulacji

## Uruchomienie
```bash
cargo run
```