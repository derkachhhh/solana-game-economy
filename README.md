# Solana Homework — Game Economy

## 📌 Опис проєкту

Цей проєкт реалізує ігрову економіку на блокчейні Solana з використанням Anchor framework.

Користувач може:
- шукати ресурси
- крафтити предмети
- обмінювати предмети на внутрішню валюту

---

## 🏗 Архітектура

Проєкт складається з 6 програм:

### 1. resource_manager
Керує ресурсами (mint / burn)

### 2. search
Дозволяє гравцю шукати ресурси (з cooldown)

### 3. crafting
Використовує ресурси для створення предметів

### 4. item_nft
Створює предмет (метадані предмета)

### 5. marketplace
Обмінює предмет на нагороду

### 6. magic_token
Зберігає баланс внутрішньої валюти

---

## 🔗 Взаємодія програм

Використовується CPI (Cross-Program Invocation):

search → resource_manager  
crafting → resource_manager + item_nft  
marketplace → magic_token  

---

## 🧠 Основні концепції

- PDA (Program Derived Address)
- CPI (Cross-Program Invocation)
- Account-based storage
- Multi-program architecture

---

## ⚙️ Запуск

```bash
anchor build
anchor test
```
---

## ✅ Результат

Тести успішно проходять:

```bash
✔ provider works
1 passing
```
---
## 📈 Перспективи розвитку

- Додавання справжніх NFT через Metaplex
- Реалізація P2P marketplace
- Інтеграція з фронтендом
---

## 👩‍💻 Автор

Деркач Єлизавета Романівна
---
