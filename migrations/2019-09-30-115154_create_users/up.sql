--ALTER DATABASE rust_actix_example SET datestyle TO "ISO, DMY";

CREATE TABLE users (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  nome VARCHAR(100) NOT NULL,
  sobrenome VARCHAR(100) NOT NULL,
  cpf CHAR(11),
  rg VARCHAR(20),
  data_nascimento TIMESTAMP,
  sexo CHAR(1),
  estado_civil VARCHAR(20),
  telefone VARCHAR(22),
  email VARCHAR(100) NOT NULL UNIQUE,
  password VARCHAR(122) NOT NULL,
  created_by VARCHAR(36) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_by VARCHAR(36) NOT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

insert into users (id, nome, sobrenome, cpf, rg, data_nascimento, sexo, estado_civil, telefone, email, password, created_by, updated_by) values 
('00000000-0000-0000-0000-000000000000', 'admin', 'user', '08673723973', '1234567', '06-06-2003', 'M', 'solteiro', '49984383188', 'admin@admin.com', '123', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000'),
('1802d2f8-1a18-43c1-9c58-1c3f7100c842', 'test', 'user', '08673723973', '1234567', '06-06-2003', 'M', 'solteiro', '49984383188', 'test@admin.com', '123', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000'),
('10000000-0000-0000-0000-000000000000','tests_main','user','08673723973','1234567','06-06-2003','M','solteiro','49984383188','test@user.com','dbf02fa3802c365d752bb431067353e534ccf0f5c14688311913f3a3313efcc1','00000000-0000-0000-0000-000000000000','00000000-0000-0000-0000-000000000000');