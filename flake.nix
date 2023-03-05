{
  inputs = { nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11"; };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      makeTest = pkgs.callPackage "${nixpkgs}/nixos/tests/make-test-python.nix";
    in {
      checks.${system}.test-diesel-migration = let
        username = "postgres";
        password = "password";
        database = "database";
      in makeTest {
        name = "test-diesel-migration";
        nodes = {
          server = { lib, config, pkgs, ... }: {
            services.postgresql = {
              enable = true;
              ensureDatabases = [ database ];
              ensureUsers = [{
                name = username;
                ensurePermissions = {
                  "DATABASE ${database}" = "ALL PRIVILEGES";
                };
              }];
              initialScript = pkgs.writeScript "initScript" ''
                ALTER USER postgres WITH PASSWORD '${password}';
              '';
            };

            systemd.services.postgresql.postStart = lib.mkAfter ''
              ${pkgs.diesel-cli}/bin/diesel migration run --database-url "postgres://${username}:${password}@localhost/${database}" --migration-dir ${self}/migrations
              # TODO: add this back
              # ${pkgs.diesel-cli}/bin/diesel migration redo --database-url "postgres://${username}:${password}@localhost/${database}" --migration-dir ${self}/migrations
              # ${pkgs.diesel-cli}/bin/diesel migration run --database-url "postgres://${username}:${password}@localhost/${database}" --migration-dir ${self}/migrations
            '';
          };
        };
        testScript = ''
          start_all()
          server.wait_for_unit("postgresql.service")
          server.succeed("sudo -u postgres -- ${pkgs.diesel-cli}/bin/diesel print-schema --database-url postgres://${username}:${password}@localhost/${database} > schema.rs")
          server.copy_from_vm("schema.rs", "")
        '';
      } {
        inherit pkgs;
        inherit (pkgs) system;
      };

      packages.${system} = {
        update-schema = pkgs.writeScriptBin "update-schema" ''
          # nix build ${self}#checks.${system}.test-diesel-migration
          BUILD_DIR=$(nix build ${self}#checks.${system}.test-diesel-migration --no-link --print-out-paths)
          rm -rf src/schema.rs
          cp $BUILD_DIR/schema.rs src/schema.rs
        '';

        run-migration = pkgs.writeScriptBin "run-migration" ''
          ${pkgs.diesel-cli}/bin/diesel migration run --migration-dir ${self}/migrations
        '';

        run-migration-based = pkgs.writeScriptBin "run-migration" ''
          ${pkgs.diesel-cli}/bin/diesel migration run --migration-dir ${self}/migrations-based
        '';

        yeet-data = pkgs.writeScriptBin "yeet-data" ''
          $PSQL -d dvbdump -c "COPY users TO '/tmp/users.csv'  WITH DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY regions TO '/tmp/regions.csv'  WITH DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY stations TO '/tmp/stations.csv'  WITH DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY trekkie_runs TO '/tmp/trekkie_runs.csv'  WITH DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY gps_points TO '/tmp/gps_points.csv'  WITH DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY raw_telegrams TO '/tmp/raw_telegrams.csv'  WITH DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY r09_telegrams TO '/tmp/r09_telegrams.csv'  WITH DELIMITER ',' CSV HEADER;"

          $PSQL -d dvbdump -c "COPY persons(*) FROM '/tmp/users.csv' DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY regions(*) FROM '/tmp/regions.csv' DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY stations(*) FROM '/tmp/stations.csv' DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY trekkie_runs(*) FROM '/tmp/stations.csv' DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY gps_points(*) FROM '/tmp/gps_points.csv' DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY raw_telegrams(*) FROM '/tmp/raw_telegrams.csv' DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY r09_telegrams(*) FROM '/tmp/r09_telegrams.csv' DELIMITER ',' CSV HEADER;"
        '';

      };

      devShells."x86_64-linux".default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [  grpc protobuf websocketpp pkg-config postgresql_14 openssl diesel-cli ];
      };
    };
}
