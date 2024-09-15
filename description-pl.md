# Tworzenie Bloga Web3 z Użyciem Polkadot i Substrate w Rust

![Polkadot i Substrate](https://example.com/polkadot_substrate_image.png)

## Wprowadzenie

Web3 to nowa generacja internetu, która opiera się na decentralizacji, blockchainie i inteligentnych kontraktach. Dzięki platformom takim jak **Polkadot** i frameworkowi **Substrate**, programiści mogą tworzyć własne blockchainy i aplikacje zdecentralizowane (dApps) w języku **Rust**.

W tym artykule pokażemy, jak stworzyć prostego bloga Web3, wykorzystując Substrate do budowy własnego blockchaina.

## Dlaczego Polkadot i Substrate?

- **Polkadot** to wielołańcuchowa platforma, która umożliwia interoperacyjność między różnymi blockchainami.
- **Substrate** to modularny framework do tworzenia niestandardowych blockchainów, który jest używany do budowy Polkadot.

**Rust** jest językiem programowania systemowego, który oferuje wysoką wydajność i bezpieczeństwo, co czyni go idealnym wyborem dla programowania blockchainów.

## Przygotowanie Środowiska

### Wymagania wstępne

- Zainstalowany **Rust** (wersja nightly)
- **Git**
- **Node.js** i **npm** (do uruchamiania interfejsu użytkownika)

### Instalacja Substrate

```bash
# Instalacja Rust nightly
rustup default nightly
rustup update
rustup target add wasm32-unknown-unknown --toolchain nightly

# Klonowanie repozytorium Substrate Node Template
git clone https://github.com/substrate-developer-hub/substrate-node-template
cd substrate-backend-template

# Kompilacja
cargo build --release
```

## Tworzenie Modułu Bloga

W Substrate, funkcjonalność blockchaina jest zorganizowana w modułach zwanych "palletami". Stworzymy własną paletę do obsługi postów na blogu.

### Struktura Palety

1. **Storage**: Miejsce przechowywania danych, takich jak posty.
2. **Events**: Wydarzenia emitowane po wykonaniu akcji.
3. **Errors**: Obsługa błędów.
4. **Calls**: Funkcje publiczne, które mogą być wywoływane przez użytkowników.

### Implementacja

#### Storage

```rust
#[pallet::storage]
#[pallet::getter(fn posts)]
pub type Posts<T: Config> = StorageMap<_, Blake2_128Concat, u64, Post<T::AccountId>>;
```

#### Struct Post

```rust
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Post<AccountId> {
    pub id: u64,
    pub title: Vec<u8>,
    pub content: Vec<u8>,
    pub author: AccountId,
}
```

#### Calls

```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn create_post(origin: OriginFor<T>, title: Vec<u8>, content: Vec<u8>) -> DispatchResult {
        let sender = ensure_signed(origin)?;

        let post_id = Self::next_post_id();
        let new_post = Post {
            id: post_id,
            title,
            content,
            author: sender.clone(),
        };

        Posts::<T>::insert(post_id, new_post);
        NextPostId::<T>::put(post_id + 1);

        Self::deposit_event(Event::PostCreated(sender, post_id));
        Ok(())
    }
}
```

## Uruchamianie Node'a

Po zaimplementowaniu palety, kompilujemy i uruchamiamy nasz node.

```bash
cargo build --release
./target/release/backend-template --dev
```

## Tworzenie Interfejsu Użytkownika

Aby użytkownicy mogli wchodzić w interakcje z naszym blockchainem, potrzebujemy interfejsu. Możemy użyć **Polkadot.js** do stworzenia prostego frontendu.

### Instalacja Dependencies

```bash
npm install @polkadot/api
```

### Łączenie z Blockchainem

```javascript
import { ApiPromise, WsProvider } from '@polkadot/api';

async function main() {
  const provider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider });

  // Wyświetlenie numeru bloku
  const blockNumber = await api.derive.chain.bestNumber();
  console.log(`Aktualny numer bloku: ${blockNumber}`);
}

main().catch(console.error);
```

## Podsumowanie

Stworzenie bloga Web3 za pomocą Polkadot i Substrate w Rust pozwala na pełne wykorzystanie możliwości decentralizacji i bezpieczeństwa, jakie oferuje blockchain. Dzięki modularnej architekturze Substrate, możemy łatwo dostosować nasz blockchain do indywidualnych potrzeb.

## Dalsze Kroki

- **Rozszerzenie funkcjonalności**: Dodanie komentarzy, systemu oceny postów.
- **Integracja z Polkadot**: Połączenie naszego blockchaina jako parachain w ekosystemie Polkadot.
- **Audyt bezpieczeństwa**: Przeprowadzenie testów i audytów w celu zapewnienia bezpieczeństwa aplikacji.

## Zasoby

- [Dokumentacja Substrate](https://substrate.dev/docs)
- [Polkadot Wiki](https://wiki.polkadot.network)
- [Rust Lang](https://www.rust-lang.org)
