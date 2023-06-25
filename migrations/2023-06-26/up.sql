
CREATE TABLE orcamento_produto (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  id_produto VARCHAR(36) NOT NULL,
  id_orcamento VARCHAR(36) NOT NULL,
  created_by VARCHAR(36) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_orcamento
   FOREIGN KEY(id_orcamento) 
   REFERENCES orcamento(id),
  CONSTRAINT fk_produto
   FOREIGN KEY(id_orcamento) 
   REFERENCES orcamento(id)
);

insert into orcamento_produto (id,
id_produto,
id_orcamento,
created_by)
VALUES
('00000000-0000-0000-0000-000000000000',
'00000000-0000-0000-0000-000000000000',
'00000000-0000-0000-0000-000000000000',
'00000000-0000-0000-0000-000000000000');