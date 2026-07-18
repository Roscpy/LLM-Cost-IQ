#!/bin/bash
set -e

echo "🔧 Phase 1 - Test d'intégration complet"

# 1. Nettoyer les conteneurs existants
echo "🧹 Nettoyage des conteneurs..."
docker-compose down -v 2>/dev/null || true

# 2. Démarrer PostgreSQL en arrière-plan
echo "🐘 Démarrage de PostgreSQL..."
docker-compose up -d postgres

# Attendre que PostgreSQL soit prêt
echo "⏳ Attente de PostgreSQL..."
until docker exec $(docker-compose ps -q postgres) pg_isready -U costiq -d costiq -q; do
  sleep 1
done
echo "✅ PostgreSQL prêt"

# 3. Exécuter les migrations (via sqlx-cli)
echo "📦 Installation de sqlx-cli..."
cargo install sqlx-cli --quiet --no-default-features --features postgres

echo "📋 Exécution des migrations..."
export DATABASE_URL=postgres://costiq:costiq123@localhost:5432/costiq
sqlx migrate run

# 4. Construire et lancer le backend en arrière-plan
echo "🦀 Construction du backend..."
cargo build --release

echo "🚀 Lancement du backend..."
cargo run --release &
BACKEND_PID=$!

# Attendre que le serveur soit prêt
echo "⏳ Attente du serveur..."
sleep 5
for i in {1..10}; do
  if curl -s http://localhost:3000/health | grep -q "OK"; then
    echo "✅ Serveur prêt"
    break
  fi
  sleep 1
done

# 5. Tester les endpoints
echo "🧪 Test de /health..."
RESP=$(curl -s http://localhost:3000/health)
if [[ "$RESP" != "OK" ]]; then
  echo "❌ /health a échoué (réponse: $RESP)"
  kill $BACKEND_PID 2>/dev/null || true
  exit 1
fi
echo "✅ /health OK"

echo "🧪 Test de /login..."
RESP=$(curl -s -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com"}')
if [[ -z "$RESP" ]] || ! echo "$RESP" | grep -q '"token"'; then
  echo "❌ /login a échoué (réponse: $RESP)"
  kill $BACKEND_PID 2>/dev/null || true
  exit 1
fi
echo "✅ /login OK (token reçu)"

# 6. Vérifier que l'utilisateur est en base
echo "🧪 Vérification en base..."
USER_COUNT=$(docker exec $(docker-compose ps -q postgres) psql -U costiq -d costiq -t -c "SELECT COUNT(*) FROM users WHERE email='test@example.com';" | tr -d ' ')
if [[ "$USER_COUNT" != "1" ]]; then
  echo "❌ Utilisateur non trouvé en base"
  kill $BACKEND_PID 2>/dev/null || true
  exit 1
fi
echo "✅ Utilisateur présent en base"

# 7. Arrêt propre
echo "🛑 Arrêt du backend..."
kill $BACKEND_PID 2>/dev/null || true
sleep 2

echo "🎉 Phase 1 validée avec succès !"
