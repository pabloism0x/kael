---
name: ml-ops
description: MLOps patterns for training pipelines, experiment tracking, model serving, and deployment. Use when building ML systems.
---

# ML-Ops Patterns

## Quick Reference

| Tool | Purpose |
|------|---------|
| MLflow | Experiment tracking, model registry |
| DVC | Data versioning |
| Weights & Biases | Experiment tracking |
| BentoML | Model serving |
| Ray | Distributed training |
| Kubeflow | ML pipelines on K8s |

## Project Structure

```
ml-project/
├── data/
│   ├── raw/                 # Original data (gitignored)
│   ├── processed/           # Processed data (gitignored)
│   └── .gitkeep
├── models/                  # Saved models (gitignored)
├── notebooks/               # Exploration notebooks
│   └── 01_eda.ipynb
├── src/
│   ├── __init__.py
│   ├── data/
│   │   ├── __init__.py
│   │   ├── dataset.py       # Dataset classes
│   │   └── preprocessing.py
│   ├── models/
│   │   ├── __init__.py
│   │   ├── base.py          # Base model interface
│   │   └── transformer.py
│   ├── training/
│   │   ├── __init__.py
│   │   ├── trainer.py
│   │   └── callbacks.py
│   ├── evaluation/
│   │   ├── __init__.py
│   │   └── metrics.py
│   └── serving/
│       ├── __init__.py
│       └── api.py
├── configs/
│   ├── model.yaml
│   └── training.yaml
├── tests/
├── dvc.yaml                 # DVC pipeline
├── pyproject.toml
└── Makefile
```

## Experiment Tracking

### MLflow Setup

```python
# src/training/trainer.py
import mlflow
from mlflow.tracking import MlflowClient

def setup_mlflow(experiment_name: str, tracking_uri: str = None):
    if tracking_uri:
        mlflow.set_tracking_uri(tracking_uri)
    mlflow.set_experiment(experiment_name)
    return MlflowClient()

def train_with_tracking(config: dict):
    with mlflow.start_run():
        # Log parameters
        mlflow.log_params(config)

        # Training loop
        model = Model(**config["model"])
        for epoch in range(config["epochs"]):
            train_loss = train_epoch(model)
            val_loss, val_metrics = validate(model)

            # Log metrics
            mlflow.log_metrics({
                "train_loss": train_loss,
                "val_loss": val_loss,
                **val_metrics,
            }, step=epoch)

        # Log model
        mlflow.pytorch.log_model(model, "model")

        # Log artifacts
        mlflow.log_artifact("configs/model.yaml")
```

### Weights & Biases

```python
import wandb

def train_with_wandb(config: dict):
    run = wandb.init(
        project="my-project",
        config=config,
        tags=["experiment", "v1"],
    )

    model = Model(**config["model"])

    for epoch in range(config["epochs"]):
        train_loss = train_epoch(model)
        val_loss, val_metrics = validate(model)

        wandb.log({
            "train_loss": train_loss,
            "val_loss": val_loss,
            **val_metrics,
            "epoch": epoch,
        })

    # Save model artifact
    artifact = wandb.Artifact("model", type="model")
    artifact.add_file("model.pt")
    run.log_artifact(artifact)

    run.finish()
```

## Data Versioning (DVC)

### dvc.yaml Pipeline

```yaml
# dvc.yaml
stages:
  preprocess:
    cmd: python -m src.data.preprocessing
    deps:
      - data/raw/
      - src/data/preprocessing.py
    outs:
      - data/processed/

  train:
    cmd: python -m src.training.trainer
    deps:
      - data/processed/
      - src/training/trainer.py
      - configs/training.yaml
    params:
      - configs/training.yaml:
          - model
          - training
    outs:
      - models/model.pt
    metrics:
      - metrics.json:
          cache: false

  evaluate:
    cmd: python -m src.evaluation.evaluate
    deps:
      - models/model.pt
      - data/processed/test/
    metrics:
      - evaluation.json:
          cache: false
```

### Commands

```bash
# Initialize DVC
dvc init

# Add data to DVC
dvc add data/raw/dataset.csv

# Run pipeline
dvc repro

# Push data to remote
dvc remote add -d storage s3://bucket/dvc
dvc push

# Pull data
dvc pull
```

## Configuration Management

### Hydra Config

```python
# configs/config.yaml
defaults:
  - model: transformer
  - training: default

model:
  hidden_size: 256
  num_layers: 4

training:
  batch_size: 32
  learning_rate: 1e-4
  epochs: 100
```

```python
# src/train.py
import hydra
from omegaconf import DictConfig

@hydra.main(version_base=None, config_path="../configs", config_name="config")
def train(cfg: DictConfig):
    print(f"Model: {cfg.model.hidden_size}")
    print(f"LR: {cfg.training.learning_rate}")

    model = build_model(cfg.model)
    trainer = Trainer(cfg.training)
    trainer.train(model)

if __name__ == "__main__":
    train()
```

```bash
# Override config via CLI
python src/train.py model.hidden_size=512 training.epochs=200
```

## Model Serving

### FastAPI Inference Server

```python
# src/serving/api.py
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import torch

app = FastAPI()

class PredictRequest(BaseModel):
    text: str

class PredictResponse(BaseModel):
    prediction: str
    confidence: float

# Load model at startup
model = None

@app.on_event("startup")
async def load_model():
    global model
    model = torch.load("models/model.pt")
    model.eval()

@app.post("/predict", response_model=PredictResponse)
async def predict(request: PredictRequest):
    if model is None:
        raise HTTPException(503, "Model not loaded")

    with torch.no_grad():
        output = model(request.text)
        prediction = output.argmax().item()
        confidence = output.softmax(-1).max().item()

    return PredictResponse(
        prediction=str(prediction),
        confidence=confidence,
    )

@app.get("/health")
async def health():
    return {"status": "healthy", "model_loaded": model is not None}
```

### BentoML Service

```python
# src/serving/service.py
import bentoml
from bentoml.io import JSON, NumpyNdarray

# Save model to BentoML
model = train_model()
bentoml.pytorch.save_model("my_model", model)

# Create service
runner = bentoml.pytorch.get("my_model:latest").to_runner()
svc = bentoml.Service("classifier", runners=[runner])

@svc.api(input=NumpyNdarray(), output=JSON())
async def predict(input_array):
    result = await runner.predict.async_run(input_array)
    return {"prediction": result.tolist()}
```

```bash
# Serve locally
bentoml serve src/serving/service.py:svc

# Build container
bentoml build
bentoml containerize my_service:latest
```

## Training Pipeline

### PyTorch Training Loop

```python
# src/training/trainer.py
from dataclasses import dataclass
from typing import Optional
import torch
from torch.utils.data import DataLoader
from tqdm import tqdm

@dataclass
class TrainerConfig:
    epochs: int = 100
    learning_rate: float = 1e-4
    batch_size: int = 32
    device: str = "cuda"
    checkpoint_dir: str = "checkpoints"
    early_stopping_patience: int = 10

class Trainer:
    def __init__(self, model, config: TrainerConfig):
        self.model = model.to(config.device)
        self.config = config
        self.optimizer = torch.optim.AdamW(
            model.parameters(),
            lr=config.learning_rate,
        )
        self.scheduler = torch.optim.lr_scheduler.CosineAnnealingLR(
            self.optimizer,
            T_max=config.epochs,
        )
        self.best_val_loss = float("inf")
        self.patience_counter = 0

    def train(self, train_loader: DataLoader, val_loader: DataLoader):
        for epoch in range(self.config.epochs):
            train_loss = self._train_epoch(train_loader)
            val_loss = self._validate(val_loader)

            self.scheduler.step()

            # Early stopping
            if val_loss < self.best_val_loss:
                self.best_val_loss = val_loss
                self.patience_counter = 0
                self._save_checkpoint(epoch, val_loss)
            else:
                self.patience_counter += 1
                if self.patience_counter >= self.config.early_stopping_patience:
                    print(f"Early stopping at epoch {epoch}")
                    break

            print(f"Epoch {epoch}: train_loss={train_loss:.4f}, val_loss={val_loss:.4f}")

    def _train_epoch(self, loader: DataLoader) -> float:
        self.model.train()
        total_loss = 0

        for batch in tqdm(loader, desc="Training"):
            self.optimizer.zero_grad()
            loss = self._compute_loss(batch)
            loss.backward()
            torch.nn.utils.clip_grad_norm_(self.model.parameters(), 1.0)
            self.optimizer.step()
            total_loss += loss.item()

        return total_loss / len(loader)

    @torch.no_grad()
    def _validate(self, loader: DataLoader) -> float:
        self.model.eval()
        total_loss = 0

        for batch in loader:
            loss = self._compute_loss(batch)
            total_loss += loss.item()

        return total_loss / len(loader)

    def _save_checkpoint(self, epoch: int, val_loss: float):
        torch.save({
            "epoch": epoch,
            "model_state_dict": self.model.state_dict(),
            "optimizer_state_dict": self.optimizer.state_dict(),
            "val_loss": val_loss,
        }, f"{self.config.checkpoint_dir}/best_model.pt")
```

## Model Registry

### MLflow Model Registry

```python
from mlflow.tracking import MlflowClient

client = MlflowClient()

# Register model
model_uri = f"runs:/{run_id}/model"
model_version = mlflow.register_model(model_uri, "my-model")

# Transition to production
client.transition_model_version_stage(
    name="my-model",
    version=model_version.version,
    stage="Production",
)

# Load production model
model = mlflow.pyfunc.load_model("models:/my-model/Production")
```

## CI/CD for ML

### GitHub Actions

```yaml
# .github/workflows/ml-pipeline.yml
name: ML Pipeline

on:
  push:
    paths:
      - 'src/**'
      - 'configs/**'
      - 'dvc.yaml'

jobs:
  train:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Python
        uses: actions/setup-python@v5
        with:
          python-version: '3.11'

      - name: Install dependencies
        run: pip install -e ".[dev]"

      - name: Setup DVC
        uses: iterative/setup-dvc@v1

      - name: Pull data
        run: dvc pull
        env:
          AWS_ACCESS_KEY_ID: ${{ secrets.AWS_ACCESS_KEY_ID }}
          AWS_SECRET_ACCESS_KEY: ${{ secrets.AWS_SECRET_ACCESS_KEY }}

      - name: Run pipeline
        run: dvc repro

      - name: Push metrics
        run: |
          dvc push
          git add dvc.lock metrics.json
          git commit -m "Update model metrics"
          git push
```

## Anti-patterns

### Avoid: Training Without Reproducibility

```python
# Bad: No seed, no config logging
model = Model()
model.fit(data)

# Good: Full reproducibility
def train(config: dict, seed: int = 42):
    set_seed(seed)
    mlflow.log_params(config)
    mlflow.log_param("seed", seed)

    model = Model(**config)
    model.fit(data)
```

### Avoid: Hardcoded Paths

```python
# Bad: Hardcoded paths
model = torch.load("/home/user/models/model.pt")

# Good: Environment-based paths
from pathlib import Path
import os

MODEL_DIR = Path(os.getenv("MODEL_DIR", "models"))
model = torch.load(MODEL_DIR / "model.pt")
```

### Avoid: No Model Versioning

```python
# Bad: Overwrite model each time
torch.save(model, "model.pt")

# Good: Version with timestamp/run_id
from datetime import datetime
timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
torch.save(model, f"models/model_{timestamp}.pt")
```
