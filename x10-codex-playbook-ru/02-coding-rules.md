# Coding Rules

## TypeScript

- Не использовать `any`, кроме случаев, где это явно обосновано комментарием.
- Все API DTO описывать типами.
- Не смешивать DTO backend и ViewModel, если требуется преобразование.
- Date/DateTime хранить как ISO string на границе API.

## React

- Server state только через TanStack Query.
- Local UI state через useState/Zustand.
- Не использовать useEffect для обычной загрузки данных.
- Формы делать контролируемо и предсказуемо.
- Ошибки API показывать пользователю.

## Tailwind

- Не использовать inline style без необходимости.
- Повторяющиеся классы выносить в компоненты.
- UI должен быть responsive.

## Phaser

- Phaser не должен вызывать API.
- Phaser получает события через Event Bus.
- Phaser не должен хранить бизнес-состояние игрока.
