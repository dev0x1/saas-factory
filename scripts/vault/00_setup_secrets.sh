#!/bin/bash

VAULT_RETRIES=5
echo "Vault is starting..."
until vault status > /dev/null 2>&1 || [ "$VAULT_RETRIES" -eq 0 ]; do
	echo "Waiting for vault to start...: $((VAULT_RETRIES--))"
	sleep 1
done

echo "Authenticating to vault..."
vault login token=$VAULT_DEV_ROOT_TOKEN_ID

echo "Initializing auth-service vault..."
vault secrets enable -version=2 -path=auth-service-secrets-kv kv
# Secrets values need to be in sync with corresponding vaules in docker-compose file, 
# while secrets keys should be same as the fields of respective structs definitions.
# For example, {'user_name', 'password'} keys for mongo creds should be same as in
# struct defined in 'services/common/src/client/db_mongo.rs::MongoClientSecrets'.
echo "Adding auth-service secrets..."
vault kv put auth-service-secrets-kv/dev/mongo user_name=test_user password=test_password
vault kv put auth-service-secrets-kv/dev/redis password=test_password

echo "Initializing user-service vault..."
vault secrets enable -version=2 -path=user-service-secrets-kv kv
# Secrets values need to be in sync with corresponding vaules in docker-compose file, 
# while secrets keys should be same as the fields of respective structs definitions.
# For example, {'user_name', 'password'} keys for mongo creds should be same as in
# struct defined in 'services/common/src/client/db_mongo.rs::MongoClientSecrets'.
echo "Adding user-service secrets..."
vault kv put user-service-secrets-kv/dev/mongo user_name=test_user password=test_password
vault kv put user-service-secrets-kv/dev/redis password=test_password

echo "Initializing tenant-service vault..."
vault secrets enable -version=2 -path=tenant-service-secrets-kv kv
# Secrets values need to be in sync with corresponding vaules in docker-compose file, 
# while secrets keys should be same as the fields of respective structs definitions.
# For example, {'user_name', 'password'} keys for mongo creds should be same as in
# struct defined in 'services/common/src/client/db_mongo.rs::MongoClientSecrets'.
echo "Adding tenant-service secrets..."
vault kv put tenant-service-secrets-kv/dev/mongo user_name=test_user password=test_password
vault kv put tenant-service-secrets-kv/dev/redis password=test_password

echo "Initializing notification-service vault..."
vault secrets enable -version=2 -path=notification-service-secrets-kv kv
echo "Adding notification-service secrets..."
vault kv put notification-service-secrets-kv/dev/redis password=test_password
vault kv put notification-service-secrets-kv/dev/smtp user_name=test_user password=test_password

echo "Done adding secrets to vault server."
