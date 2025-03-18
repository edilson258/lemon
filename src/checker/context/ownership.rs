use super::value::ValueId;
use std::collections::HashMap;

/// Identificador único para um empréstimo (borrow).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwnershipId(pub(crate) usize);

impl OwnershipId {
	pub fn as_usize(&self) -> usize {
		self.0
	}
}

/// Representa um empréstimo de um valor.
/// Pode ser mutável ou imutável.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OwnershipEntry {
	pub id: OwnershipId,
	pub is_mutable: bool,
	pub value_id: ValueId,
}

impl OwnershipEntry {
	pub fn new(id: OwnershipId, is_mutable: bool, value_id: ValueId) -> Self {
		Self { id, is_mutable, value_id }
	}

	pub fn is_mutable(&self) -> bool {
		self.is_mutable
	}
}

/// Pilha de controle de ownership para cada valor.
/// Garante que os empréstimos sigam as regras de ownership.
#[derive(Debug, Clone)]
pub struct OwnershipStack {
	stack: Vec<OwnershipEntry>,
}

impl OwnershipStack {
	pub fn new() -> Self {
		Self { stack: Vec::new() }
	}

	/// Adiciona um novo empréstimo na pilha.
	pub fn push(&mut self, borrow: OwnershipEntry) {
		self.stack.push(borrow);
	}

	/// Remove o empréstimo do topo da pilha.
	pub fn pop(&mut self) -> Option<OwnershipEntry> {
		self.stack.pop()
	}

	/// Obtém o empréstimo que está no topo da pilha.
	pub fn top(&self) -> Option<&OwnershipEntry> {
		self.stack.last()
	}

	/// Verifica se um novo empréstimo pode ser criado.
	pub fn is_borrow_allowed(&self, is_mutable: bool) -> bool {
		if let Some(top_borrow) = self.top() {
			if is_mutable {
				return false; // Não pode criar um empréstimo mutável se qualquer outro já existir.
			}
			if top_borrow.is_mutable() {
				return false; // Não pode criar um empréstimo imutável se o topo é mutável.
			}
		}
		true
	}

	/// Verifica se a pilha contém um determinado empréstimo.
	pub fn contains(&self, ownership_id: OwnershipId) -> bool {
		self.stack.iter().any(|b| b.id == ownership_id)
	}
}

/// Gerencia os empréstimos de ownership de valores.
/// Cada `ValueId` pode ter sua própria pilha de empréstimos.
#[derive(Debug, Clone)]
pub struct OwnershipManager {
	ownerships: HashMap<ValueId, OwnershipStack>,
	next_ownership_id: usize,
}

impl OwnershipManager {
	pub fn new() -> Self {
		Self { ownerships: HashMap::new(), next_ownership_id: 0 }
	}

	/// **Cria um novo empréstimo para um valor (`ValueId`).**
	/// - Retorna `Some(OwnershipId)` se o empréstimo foi bem-sucedido.
	/// - Retorna `None` se o empréstimo for inválido (violando ownership).
	pub fn create_borrow(&mut self, value_id: ValueId, is_mutable: bool) -> Option<OwnershipId> {
		let stack = self.ownerships.entry(value_id).or_insert_with(OwnershipStack::new);

		if !stack.is_borrow_allowed(is_mutable) {
			return None; // Não pode criar esse empréstimo.
		}

		let ownership_id = OwnershipId(self.next_ownership_id);
		self.next_ownership_id += 1;

		let borrow = OwnershipEntry::new(ownership_id, is_mutable, value_id);
		stack.push(borrow);

		Some(ownership_id)
	}

	/// **Obtém um empréstimo existente pelo ID.**
	pub fn get_borrow(&self, ownership_id: OwnershipId) -> Option<&OwnershipEntry> {
		self.ownerships.values().find_map(|stack| stack.stack.iter().find(|b| b.id == ownership_id))
	}

	/// **Libera um empréstimo (retornando ao sucessor na pilha).**
	/// - Retorna `true` se o empréstimo foi removido com sucesso.
	/// - Retorna `false` se o empréstimo não foi encontrado.
	pub fn release_borrow(&mut self, ownership_id: OwnershipId) -> bool {
		for (_, stack) in self.ownerships.iter_mut() {
			if stack.contains(ownership_id) {
				stack.pop();
				return true;
			}
		}
		false
	}

	/// **Verifica se um novo empréstimo pode ser criado para um valor (`ValueId`).**
	pub fn is_borrow_allowed(&self, value_id: ValueId, is_mutable: bool) -> bool {
		if let Some(stack) = self.ownerships.get(&value_id) {
			stack.is_borrow_allowed(is_mutable)
		} else {
			true // Nenhum empréstimo ativo, pode criar um novo.
		}
	}
}

impl Default for OwnershipManager {
	fn default() -> Self {
		Self::new()
	}
}
