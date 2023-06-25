
CREATE TABLE orcamento_produtos (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  id_produto VARCHAR(36) NOT NULL,
  id_orcamento VARCHAR(36) NOT NULL,
  quantidade INT NOT NULL,
  created_by VARCHAR(36) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_orcamentos
   FOREIGN KEY(id_orcamento) 
   REFERENCES orcamentos(id),
  CONSTRAINT fk_produto
   FOREIGN KEY(id_produto) 
   REFERENCES produtos(id)
);

insert into orcamento_produtos (id,
id_produto,
id_orcamento,
quantidade,
created_by)
VALUES
('00000000-0000-0000-0000-000000000000',
'00000000-0000-0000-0000-000000000000',
'00000000-0000-0000-0000-000000000000',
1,
'00000000-0000-0000-0000-000000000000');