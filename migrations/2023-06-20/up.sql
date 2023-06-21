
CREATE TABLE clientes (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  nome VARCHAR(100) NOT NULL,
  sobrenome VARCHAR(100) NOT NULL,
  cpf CHAR(11),
  rg VARCHAR(20),
  data_nascimento TIMESTAMP,
  sexo CHAR(1),
  estado_civil VARCHAR(20),
  telefone VARCHAR(22),
  created_by VARCHAR(36) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_by VARCHAR(36) NOT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

insert into clientes (id, nome, sobrenome, cpf, rg, data_nascimento, sexo, estado_civil, telefone, created_by, updated_by) values 
('00000000-0000-0000-0000-000000000000', 'Nikolas', 'Lunkes', '08673723973', '1234567', '06-06-2003', 'M', 'solteiro', '49984383188', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000')