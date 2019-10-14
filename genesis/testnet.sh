
# node 0
# Local node identity is: Qme8GBwAdp4SskxYiKeMo9yuX5PnQZBXvfNrVhQzJLTE2v
# authority key 5GCezrbwQU8SbAceXwH6KZc3pzNhW9pxzV4FCHgVgnDGyEpT
# Discovered external node address: /ip4/xx.xx.x.xx/tcp/30333/p2p/Qme8GBwAdp4SskxYiKeMo9yuX5PnQZBXvfNrVhQzJLTE2v
# Listening for new connections on 127.0.0.1:9944

./target/release/cennznet \
  --base-path /tmp/Andrea \
  --chain ./rimu-latest.json \
  --key //Andrea \
  --port 30333 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator
  --name AndreaNode

# node 1
# Local node identity is: QmTBkfxmXshXdjGD24JvnexC7G6TenPLSpdSPbM4Y9hELM
# authority key 5EcZF6Qh6PjAzCXAJrQnp4Pa6nBCSp41mRyQev2mUhrp6CfZ
# Discovered external node address: /ip4/10.xx.xx.xx/tcp/30334/p2p/QmTBkfxmXshXdjGD24JvnexC7G6TenPLSpdSPbM4Y9hELM
# Discovered external node address: /ip4/xx.xx.x.xx/tcp/30334/p2p/QmTBkfxmXshXdjGD24JvnexC7G6TenPLSpdSPbM4Y9hELM
# Listening for new connections on 127.0.0.1:62797

./target/release/cennznet \
  --base-path /tmp/Brooke \
  --chain ./rimu-latest.json \
  --key //Brooke \
  --port 30334 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator \
  --name BrookeNode \
  --bootnodes /ip4/xx.xx.x.xx/tcp/30333/p2p/Qme8GBwAdp4SskxYiKeMo9yuX5PnQZBXvfNrVhQzJLTE2v

# node 2
# Local node identity is: QmSJ3jN6xa6dNGSvbNmGDdpdW16hB3rZb1PKsfK7ChVhef
# authority key 5HbBTwffvRbWNyxx5xq7RQPXF381AzBasUojJkLWHuYY97Ke
# Discovered external node address: /ip4/10.xx.xx.xx/tcp/30335/p2p/QmSJ3jN6xa6dNGSvbNmGDdpdW16hB3rZb1PKsfK7ChVhef
# Discovered external node address: /ip4/xx.xx.x.xx/tcp/30335/p2p/QmSJ3jN6xa6dNGSvbNmGDdpdW16hB3rZb1PKsfK7ChVhef
# Listening for new connections on 127.0.0.1:50895.
./target/release/cennznet \
  --base-path /tmp/Courtney \
  --chain ./rimu-latest.json \
  --key //Courtney \
  --port 30335 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator \
  --name CourtneyNode \
  --bootnodes /ip4/xx.xx.x.xx/tcp/30333/p2p/Qme8GBwAdp4SskxYiKeMo9yuX5PnQZBXvfNrVhQzJLTE2v
  --bootnodes /ip4/xx.xx.x.xx/tcp/30334/p2p/QmTBkfxmXshXdjGD24JvnexC7G6TenPLSpdSPbM4Y9hELM

  # node 3
  # Local node identity is: QmSxbF9wtM1krfwMLyxkDR9Tqn4Go2mWezfBTjNWg4RAEa
  # Discovered external node address: /ip4/10.xx.xx.xx/tcp/30336/p2p/QmSxbF9wtM1krfwMLyxkDR9Tqn4Go2mWezfBTjNWg4RAEa
  # Discovered external node address: /ip4/xx.xx.x.xx/tcp/30336/p2p/QmSxbF9wtM1krfwMLyxkDR9Tqn4Go2mWezfBTjNWg4RAEa

  ./target/release/cennznet \
  --base-path /tmp/Drew \
  --chain ./rimu-latest.json \
  --key //Drew \
  --port 30336 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator \
  --name DrewNode \
  --bootnodes /ip4/xx.xx.x.xx/tcp/30333/p2p/Qme8GBwAdp4SskxYiKeMo9yuX5PnQZBXvfNrVhQzJLTE2v
  --bootnodes /ip4/xx.xx.x.xx/tcp/30335/p2p/QmSJ3jN6xa6dNGSvbNmGDdpdW16hB3rZb1PKsfK7ChVhef
